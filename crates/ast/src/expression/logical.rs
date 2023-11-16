use crate::{Expression, Node};

#[derive(Debug, Clone)]
pub struct Logical {
	pub left: Box<Expression>,
	pub op: Node<LogicalOperator>,
	pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum LogicalOperator {
	And,
	Or,
}
