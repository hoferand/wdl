use crate::{Expression, Node};

/// Represents a logical expression.
///
/// Syntax:  
/// [`Expression`] [`LogicOperator`] [`Expression`]
#[derive(Debug, Clone)]
pub struct Logic {
	pub left: Box<Expression>,
	pub op: Node<LogicOperator>,
	pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicOperator {
	And,
	Or,
}
