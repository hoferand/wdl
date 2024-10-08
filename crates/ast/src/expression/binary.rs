use crate::{Expression, Node};

/// Represents a binary operation.
///
/// Syntax:  
/// [`Expression`] [`BinaryOperator`] [`Expression`]
#[derive(Debug, Clone)]
pub struct Binary {
	pub left: Box<Expression>,
	pub op: Node<BinaryOperator>,
	pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
