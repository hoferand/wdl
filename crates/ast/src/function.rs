use crate::{Block, IdentfierTyped, Node, Type};

pub struct Function {
	pub parameter: Vec<Node<IdentfierTyped>>,
	pub return_type: Node<Type>,
	pub body: Node<Block>,
}
