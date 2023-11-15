use crate::{Expression, Node};

pub struct Binary {
	pub left: Box<Expression>,
	pub op: Node<BinaryOperator>,
	pub right: Box<Expression>,
}

pub enum BinaryOperator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulo,
	Equal,
	NotEqual,
	Less,
	LessEqual,
	Greater,
	GreaterEqual,
}
