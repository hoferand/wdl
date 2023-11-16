use crate::{FunctionDeclaration, GlobalDeclaration, Import, Node, Order};

#[derive(Debug, Clone)]
pub struct Workflow {
	pub imports: Vec<Node<Import>>,
	pub globals: Vec<Node<GlobalDeclaration>>,
	pub order: Node<Order>,
	pub functions: Vec<Node<FunctionDeclaration>>,
}
