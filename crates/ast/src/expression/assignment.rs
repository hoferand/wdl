use crate::{Expression, Identifier, Node};

pub struct Assignment {
	pub id: Node<Identifier>,
	pub value: Box<Expression>,
}
