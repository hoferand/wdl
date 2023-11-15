use crate::{Expression, Node};

pub struct Logical {
	pub left: Box<Expression>,
	pub op: Node<LogicalOperator>,
	pub right: Box<Expression>,
}

pub enum LogicalOperator {
	And,
	Or,
}
