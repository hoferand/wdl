use crate::{Expression, Identifier, Node};

#[derive(Debug, Clone)]
pub struct Assignment {
	pub id: Node<Identifier>,
	pub value: Box<Expression>,
}
