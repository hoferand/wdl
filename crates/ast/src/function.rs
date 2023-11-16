use crate::{Block, IdentifierTyped, Node, Type};

#[derive(Debug, Clone)]
pub struct Function {
	pub parameter: Vec<Node<IdentifierTyped>>,
	pub return_type: Node<Type>,
	pub body: Node<Block>,
}
