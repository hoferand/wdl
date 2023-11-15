use crate::{Expression, Node};

pub struct Unary {
	pub op: Node<UnaryOperator>,
	pub right: Box<Expression>,
}

pub enum UnaryOperator {
	Negate,
	Flip,
}
