use clap::Parser;

use ast::{Block, Expression, Literal, Node, Order, Print, Sleep, Span, Statement, Workflow};
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

async fn run(_file: &str) {
	// TODO: read wf from file, lex and parse it
	println!("Run:");
	let wf = Workflow {
		imports: Vec::new(),
		globals: Vec::new(),
		order: Node {
			span: Span::default(),
			val: Order {
				block: Node {
					span: Span::default(),
					val: Block {
						stmts: vec![
							Statement::Sleep(Node {
								span: Span::default(),
								val: Sleep {
									time: Node {
										span: Span::default(),
										val: Expression::Literal(Node {
											span: Span::default(),
											val: Literal::Number(2000.0),
										}),
									},
								},
							}),
							Statement::Print(Node {
								span: Span::default(),
								val: Print {
									value: Expression::Literal(Node {
										span: Span::default(),
										val: Literal::String("Hello World!".to_owned()),
									}),
								},
							}),
						],
					},
				},
			},
		},
		functions: Vec::new(),
	};

	let interpreter = Interpreter::new(&wf);

	let Ok(interpreter) = interpreter.init(Vec::new()).await else {
		panic!("failed to init interpreter");
	};

	if interpreter.run().await.is_err() {
		panic!("failed to run workflow");
	};
}
