use std::collections::HashMap;
use std::process::exit;

use clap::Parser;
use tokio::fs::read_to_string;

use ast::Identifier;
use logger::error;
use logger::Colorize;

use interpreter::Interpreter;

#[derive(Debug, Parser)]
enum Cli {
	#[clap(name = "run", about = "Run the program")]
	Run {
		file: String,
		variables: Vec<String>,
	},
	#[clap(name = "fmt", about = "Format the program")]
	Fmt { file: String },
	#[clap(name = "check", about = "Check the program")]
	Check { file: String },
	#[clap(name = "router", about = "Simulates the router")]
	Router,
}

#[tokio::main]
async fn main() {
	match Cli::parse() {
		Cli::Run { file, variables } => run(&file, variables).await,
		Cli::Fmt { .. } => todo!(),
		Cli::Check { .. } => todo!(),
		Cli::Router => todo!(),
	}
}

async fn run(file: &str, vars: Vec<String>) -> ! {
	let mut variables = HashMap::new();
	for var in vars {
		let Some(parts) = var.split_once('=') else {
			error!(
				"Variable malformed `{}`, expected format <identifier>=<JSON value>!",
				var
			);
			exit(1);
		};

		let id = Identifier(parts.0.to_owned());
		let Ok(val) = serde_json::from_str::<serde_json::Value>(parts.1) else {
			error!(
				"Invalid variable value `{}`, cannot be deserialized!",
				parts.1
			);
			exit(1);
		};

		variables.insert(id, val);
	}

	let Ok(src_code) = read_to_string(file).await else {
		error!("Failed to read source file!");
		exit(1);
	};
	let workflow = match parser::get_ast(&src_code) {
		Ok(wf) => wf,
		Err(error) => {
			print_parser_error(&error);
			exit(1);
		}
	};

	println!("Run:");
	let interpreter = Interpreter::new(&workflow);

	let interpreter = match interpreter.init(variables).await {
		Ok(i) => i,
		Err(error) => {
			print_interpreter_error(&error);
			exit(1);
		}
	};

	if let Err(error) = interpreter.run().await {
		print_interpreter_error(&error);
		exit(1);
	};

	exit(0);
}

fn print_interpreter_error(error: &interpreter::Error) {
	dbg!(error);
}

fn print_parser_error(error: &parser::Error) {
	dbg!(error);
}
