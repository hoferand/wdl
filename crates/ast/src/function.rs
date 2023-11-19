use crate::{Block, Identifier, Node};

#[derive(Debug, Clone)]
pub struct Function {
	pub parameter: Vec<Node<Parameter>>,
	//pub return_type: Node<Type>,
	pub body: Node<Block>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
	pub id: Node<Identifier>,
	//pub type_: Node<Type>,
}
