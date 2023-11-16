use crate::{FunctionDeclaration, Import, Node};

#[derive(Debug, Clone)]
pub struct Module {
	pub imports: Vec<Node<Import>>,
	pub functions: Vec<Node<FunctionDeclaration>>,
}
