use crate::{Expression, Identifier, Node};

#[derive(Debug, Clone)]
pub struct Member {
	pub object: Box<Expression>,
	pub member: Node<Identifier>,
}
