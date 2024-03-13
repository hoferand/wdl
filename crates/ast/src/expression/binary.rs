use serde::{Deserialize, Serialize};

use crate::{Expression, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binary {
	pub left: Box<Expression>,
	pub op: Node<BinaryOperator>,
	pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulo,
	NullCoalescing,
	Equal,
	NotEqual,
	Less,
	LessEqual,
	Greater,
	GreaterEqual,
}
