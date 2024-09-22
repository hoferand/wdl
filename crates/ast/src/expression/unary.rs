use crate::{Expression, Node};

/// Represents a unary operation.
///
/// Syntax:  
/// [`UnaryOperator`] [`Expression`]
#[derive(Debug, Clone)]
pub struct Unary {
	pub op: Node<UnaryOperator>,
	pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
	Negate,
	Flip,
	Receive,
}
