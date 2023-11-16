use clap::Parser;
use tokio::fs::read_to_string;

use interpreter::Interpreter;

#[derive(Debug, Parser)]
enum Cli {
	#[clap(name = "run", about = "Run the program")]
	Run { file: String },
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
		Cli::Run { file } => run(&file).await,
		Cli::Fmt { .. } => todo!(),
		Cli::Check { .. } => todo!(),
		Cli::Router => todo!(),
	}
}

async fn run(file: &str) {
	let src_code = read_to_string(file).await.expect("Failed to read file!");
	let workflow = match parser::get_ast(&src_code) {
		Ok(wf) => wf,
		Err(error) => {
			print_parser_error(&error);
			std::process::exit(1);
		}
	};

	println!("Run:");
	let interpreter = Interpreter::new(&workflow);

	let interpreter = match interpreter.init(Vec::new()).await {
		Ok(i) => i,
		Err(error) => {
			print_interpreter_error(&error);
			std::process::exit(1);
		}
	};

	if let Err(error) = interpreter.run().await {
		print_interpreter_error(&error);
		std::process::exit(1);
	};
}

fn print_interpreter_error(error: &interpreter::Error) {
	dbg!(error);
}

fn print_parser_error(error: &parser::Error) {
	dbg!(error);
}
