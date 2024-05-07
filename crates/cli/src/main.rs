use std::process::ExitCode;
use std::{collections::HashMap, error::Error};

use clap::Parser;
use log::{error, info, warn, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode}; // cspell:disable-line
use tokio::fs;
use tokio::fs::read_to_string;

use ast::Identifier;
use common::{convert_parser_error, create_error_location, Target};

mod router;
use router::router;

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
async fn main() -> Result<ExitCode, Box<dyn Error>> {
	TermLogger::init(
		LevelFilter::Info,
		Config::default(),
		TerminalMode::Stderr,
		ColorChoice::Auto,
	)?;

	match Cli::parse() {
		Cli::Run { file, variables } => run(&file, variables).await,
		Cli::Compile { file } => compile(&file).await,
		Cli::Fmt { .. } => todo!(),
		Cli::Check { file } => check(&file).await,
		Cli::Router => router().await,
	}
}

async fn run(file: &str, vars: Vec<String>) -> Result<ExitCode, Box<dyn Error>> {
	let mut variables = HashMap::new();
	for var in vars {
		let Some(parts) = var.split_once('=') else {
			error!(
				"Variable malformed `{}`, expected format <identifier>=<JSON value>!",
				var
			);
			return Ok(ExitCode::FAILURE);
		};

		let id = Identifier {
			id: parts.0.to_owned(),
		};
		let Ok(val) = serde_json::from_str(parts.1) else {
			error!(
				"Invalid variable value `{}`, cannot be deserialized!",
				parts.1
			);
			return Ok(ExitCode::FAILURE);
		};

		variables.insert(id, val);
	}

	let src_code = read_to_string(file).await?;
	let workflow = match parser::get_ast(&src_code) {
		Ok(wf) => wf,
		Err(error) => {
			print_parser_error(&error, &src_code);
			return Ok(ExitCode::FAILURE);
		}
	};

	let order = match interpreter::start_workflow(workflow, variables).await {
		Ok(o) => o,
		Err(error) => {
			print_interpreter_error(&error, &src_code);
			return Ok(ExitCode::FAILURE);
		}
	};

	if let Err(error) = interpreter::run_order(order).await {
		print_interpreter_error(&error, &src_code);
		return Ok(ExitCode::FAILURE);
	}

	Ok(ExitCode::SUCCESS)
}

async fn compile(file: &str) -> Result<ExitCode, Box<dyn Error>> {
	let src_code = read_to_string(file).await?;
	let workflow = match parser::get_ast(&src_code) {
		Ok(wf) => wf,
		Err(error) => {
			print_parser_error(&error, &src_code);
			return Ok(ExitCode::FAILURE);
		}
	};

	let json = serde_json::to_string_pretty(&workflow)?;

	fs::write(format!("{}.compiled", file), json).await?;

	Ok(ExitCode::SUCCESS)
}

async fn check(file: &str) -> Result<ExitCode, Box<dyn Error>> {
	let src_code = read_to_string(file).await?;
	if let Err(error) = parser::get_ast(&src_code) {
		print_parser_error(&error, &src_code);
		return Ok(ExitCode::FAILURE);
	};

	Ok(ExitCode::SUCCESS)
}

fn print_interpreter_error(error: &interpreter::Error, src_code: &str) {
	match &error.kind {
		interpreter::ErrorKind::Fatal(msg) => error!("{}!", msg),
		interpreter::ErrorKind::VariableAlreadyInUse { id } => {
			error!("Variable `{}` already in use!", id.id);
		}
		interpreter::ErrorKind::VariableNotFound { id } => {
			error!("Variable `{}` not found!", id.to_string());
		}
		interpreter::ErrorKind::InvalidType { msg } => {
			error!("Invalid types, {}!", msg);
		}
		interpreter::ErrorKind::DivisionByZero => {
			error!("Division by zero!");
		}
		interpreter::ErrorKind::ArityMismatch { expected, given } => {
			error!(
				"Invalid count of function call parameter, expected `{}`, given `{}`!",
				expected, given
			);
		}
		interpreter::ErrorKind::MissingArgument { id } => {
			error!("Argument `{}` missing!", id);
		}
		interpreter::ErrorKind::UnknownArgument { id } => {
			error!("Named argument `{}` unknown!", id);
		}
		interpreter::ErrorKind::OrderDone => {
			info!("Order done!");
		}
		interpreter::ErrorKind::OrderCancel => {
			warn!("Order canceled!");
		}
	}

	if let Some(ref span) = error.src {
		eprintln!(
			"{}",
			create_error_location(&span.start, &span.end, src_code, Target::ANSI)
		);
	}
}

fn print_parser_error(error: &parser::Error, src_code: &str) {
	for error in convert_parser_error(error, src_code, Target::ANSI) {
		error!("{}", error.title);
		if let Some(pos) = error.pos {
			eprintln!("{}", pos.span_str);
		}
	}
}
