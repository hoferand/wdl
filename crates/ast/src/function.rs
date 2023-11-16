use crate::{Block, Identifier, Node, Type};

#[derive(Debug, Clone)]
pub struct Function {
	pub parameter: Vec<Node<Argument>>,
	pub return_type: Node<Type>,
	pub body: Node<Block>,
}

#[derive(Debug, Clone)]
pub struct Argument {
	pub id: Node<Identifier>,
	pub type_: Node<Type>,
}
