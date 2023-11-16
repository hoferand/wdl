use crate::{Expression, Node};

#[derive(Debug, Clone)]
pub struct Unary {
	pub op: Node<UnaryOperator>,
	pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
	Negate,
	Flip,
}
