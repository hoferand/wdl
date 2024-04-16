use std::collections::HashMap;
use std::process::ExitCode;

use clap::Parser;
use tokio::fs;
use tokio::fs::read_to_string;

use ast::{Identifier, Location};
use logger::error;
use logger::Colorize;

#[derive(Debug, Parser)]
enum Cli {
	#[clap(name = "run", about = "Run the program")]
	Run {
		file: String,
		variables: Vec<String>,
	},
	#[clap(name = "compile", about = "Compile the program")]
	Compile { file: String },
	#[clap(name = "fmt", about = "Format the program")]
	Fmt { file: String },
	#[clap(name = "check", about = "Check the program")]
	Check { file: String },
	#[clap(name = "router", about = "Simulates the router")]
	Router,
}

#[tokio::main]
async fn main() -> ExitCode {
	match Cli::parse() {
		Cli::Run { file, variables } => run(&file, variables).await,
		Cli::Compile { file } => compile(&file).await,
		Cli::Fmt { .. } => todo!(),
		Cli::Check { .. } => todo!(),
		Cli::Router => todo!(),
	}
}

async fn run(file: &str, vars: Vec<String>) -> ExitCode {
	let mut variables = HashMap::new();
	for var in vars {
		let Some(parts) = var.split_once('=') else {
			error!(
				"Variable malformed `{}`, expected format <identifier>=<JSON value>!",
				var
			);
			return ExitCode::FAILURE;
		};

		let id = Identifier(parts.0.to_owned());
		let Ok(val) = serde_json::from_str::<serde_json::Value>(parts.1) else {
			error!(
				"Invalid variable value `{}`, cannot be deserialized!",
				parts.1
			);
			return ExitCode::FAILURE;
		};

		variables.insert(id, val);
	}

	let src_code = match read_to_string(file).await {
		Ok(content) => content,
		Err(err) => {
			error!("Failed to read source file, {}!", err.kind());
			return ExitCode::FAILURE;
		}
	};
	let workflow = match parser::get_ast(&src_code) {
		Ok(wf) => wf,
		Err(error) => {
			print_parser_error(&error, &src_code);
			return ExitCode::FAILURE;
		}
	};

	let order = match interpreter::start_workflow(workflow, variables).await {
		Ok(o) => o,
		Err(error) => {
			print_interpreter_error(&error, &src_code);
			return ExitCode::FAILURE;
		}
	};

	if let Err(error) = interpreter::run_order(order).await {
		print_interpreter_error(&error, &src_code);
		return ExitCode::FAILURE;
	}

	ExitCode::SUCCESS
}

async fn compile(file: &str) -> ExitCode {
	let src_code = match read_to_string(file).await {
		Ok(content) => content,
		Err(err) => {
			error!("Failed to read source file, {}!", err.kind());
			return ExitCode::FAILURE;
		}
	};
	let workflow = match parser::get_ast(&src_code) {
		Ok(wf) => wf,
		Err(error) => {
			print_parser_error(&error, &src_code);
			return ExitCode::FAILURE;
		}
	};

	let Ok(json) = serde_json::to_string_pretty(&workflow) else {
		error!("Failed to stringify compiled workflow!");
		return ExitCode::FAILURE;
	};

	if fs::write(format!("{}.compiled", file), json).await.is_err() {
		error!("Failed to write compiled workflow to file!");
		return ExitCode::FAILURE;
	}

	ExitCode::SUCCESS
}

fn print_interpreter_error(error: &interpreter::Error, src_code: &str) {
	match error {
		interpreter::Error::Fatal(msg) => error!("{}!", msg),
		interpreter::Error::VariableAlreadyInUse { id, span } => {
			error!("Variable `{}` already in use!", id.0);
			print_error_location(&span.start, &span.end, src_code);
		}
		interpreter::Error::VariableNotFound { id, span } => {
			error!("Variable `{}` not found!", id.to_string());
			print_error_location(&span.start, &span.end, src_code);
		}
		interpreter::Error::InvalidType { msg, span } => {
			error!("Invalid types, {}!", msg);
			print_error_location(&span.start, &span.end, src_code);
		}
		interpreter::Error::DivisionByZero { span } => {
			error!("Division by zero!");
			print_error_location(&span.start, &span.end, src_code);
		}
		interpreter::Error::ArityMismatch {
			expected,
			given,
			span,
		} => {
			error!(
				"Invalid count of function call parameter, expected `{}`, given `{}`!",
				expected, given
			);
			print_error_location(&span.start, &span.end, src_code);
		}
		interpreter::Error::TooFewArguments { span } => {
			error!("Too few arguments given for function call!");
			print_error_location(&span.start, &span.end, src_code);
		}
	}
}

fn print_parser_error(error: &parser::Error, src_code: &str) {
	match error {
		parser::Error::Lexer(errors) => {
			for err in errors {
				match err {
					parser::LexerError::InvalidCharacter { char, loc } => {
						error!("Invalid character `{}` found!", char);
						print_error_location(
							loc,
							&Location {
								line: loc.line,
								column: loc.column + 1,
							},
							src_code,
						);
					}
					parser::LexerError::InvalidNumber { src, span } => {
						error!("Invalid number `{}` found!", src);
						print_error_location(&span.start, &span.end, src_code);
					}
					parser::LexerError::UnexpectedEndOfFile => {
						error!("Unexpected end of file!");
					}
					parser::LexerError::InvalidEscape { char, loc } => {
						error!("Invalid character escape `\\{}`", char);
						print_error_location(
							&Location {
								line: loc.line,
								column: loc.column - 1,
							},
							&Location {
								line: loc.line,
								column: loc.column + 1,
							},
							src_code,
						);
					}
				}
			}
		}
		parser::Error::Parser(err) => match err {
			parser::ParserError::Fatal(msg) => error!("{}!", msg),
			parser::ParserError::UnexpectedToken { src, span } => {
				error!("Unexpected token `{}`!", src);
				print_error_location(&span.start, &span.end, src_code);
			}
			parser::ParserError::SecondActions { actions1, actions2 } => {
				error!("Multiple actions blocks found!");
				println!("First block:");
				print_error_location(&actions1.start, &actions1.end, src_code);
				println!("Second block:");
				print_error_location(&actions2.start, &actions2.end, src_code);
			}
			parser::ParserError::UnexpectedEoF => error!("Unexpected end of file!"),
		},
	}
}

fn print_error_location(start: &Location, end: &Location, src: &str) {
	let mut lines: Vec<&str> = src.lines().collect();
	lines.push("");

	let number_padding = (end.line + 1).to_string().len();
	println!("{:>pad$}", "|".blue(), pad = number_padding + 2);

	let show_lines = 3;
	let mut skipped = false;
	for line_number in start.line..=end.line {
		if (end.line - start.line) > show_lines * 2
			&& !(line_number - start.line < show_lines || end.line - line_number < show_lines)
		{
			if !skipped {
				skipped = true;
				println!(
					"{:>pad$} {}",
					"|".blue(),
					"...".blue(),
					pad = number_padding + 2
				);
			}
		} else if let Some(line) = lines.get(line_number) {
			println!(
				"{:<pad$} {:} {}",
				(line_number + 1).to_string().blue(),
				"|".blue(),
				line,
				pad = number_padding,
			);
		} else {
			error!(
				"Internal error, line number `{}` does not exist!",
				line_number + 1
			);
		}
	}

	println!("{:>pad$}", "|".blue(), pad = number_padding + 2);
}
