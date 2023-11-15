use ast::{Block, Node, Order, Span, Workflow};
use interpreter::Interpreter;

#[tokio::main]
async fn main() {
	// run
	// fmt
	// check

	// router

	let wf = Workflow {
		imports: Vec::new(),
		globals: Vec::new(),
		order: Node {
			span: Span::default(),
			val: Order {
				block: Node {
					span: Span::default(),
					val: Block { stmts: Vec::new() },
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
