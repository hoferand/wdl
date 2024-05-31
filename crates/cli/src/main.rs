use std::process::ExitCode;
use std::{collections::HashMap, error::Error};

use clap::Parser;
use log::{error, info, warn, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode}; // cspell:disable-line
use tokio::fs::read_to_string;
use tokio::sync::mpsc;

use ::router::RouterClientGrpc;
use ast::Identifier;
use format::{format_parser_error, ColorMode};
use interpreter::LogEntry;

mod router;
use router::router;

#[derive(Debug, Parser)]
enum Cli {
	#[clap(name = "run", about = "Run the program")]
	Run {
		file: String,
		variables: Vec<String>,
	},
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

	let (user_log_sender, mut user_log_receiver) = mpsc::channel::<LogEntry>(10);

	let log_handle = tokio::spawn(async move {
		while let Some(log) = user_log_receiver.recv().await {
			eprintln!("[{}]{}", log.level, log.msg);
		}
	});

	let ret = interpreter::run_workflow(
		workflow,
		variables,
		interpreter::Router::Grpc(RouterClientGrpc),
		user_log_sender,
	)
	.await;

	if let Err(err) = log_handle.await {
		error!("Failed to wait for log receiver: `{}`", err);
	};

	if let Err(error) = ret {
		print_interpreter_error(&error, &src_code);
		return Ok(ExitCode::FAILURE);
	}

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

fn print_interpreter_error(err: &interpreter::Error, src_code: &str) {
	let error = format::format_interpreter_error(err, src_code, ColorMode::ANSI);
	let error_loc = match error.pos {
		None => String::new(),
		Some(pos) => format!("\n{}", pos.span_str),
	};
	match err.kind {
		interpreter::ErrorKind::OrderDone => {
			info!("{}{}", error.title, error_loc);
		}
		interpreter::ErrorKind::OrderCancel => {
			warn!("{}{}", error.title, error_loc);
		}
		_ => {
			error!("{}{}", error.title, error_loc);
		}
	}
}

fn print_parser_error(error: &parser::Error, src_code: &str) {
	for error in format_parser_error(error, src_code, ColorMode::ANSI) {
		if let Some(pos) = error.pos {
			error!("{}\n{}", error.title, pos.span_str);
		} else {
			error!("{}", error.title);
		}
	}
}
