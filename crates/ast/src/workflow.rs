use crate::{FunctionDeclaration, GlobalDeclaration, Node, Order};

#[derive(Debug, Clone)]
pub struct Workflow {
	pub globals: Vec<Node<GlobalDeclaration>>,
	pub order: Node<Order>,
	pub functions: Vec<Node<FunctionDeclaration>>,
}
