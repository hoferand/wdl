use serde::{Deserialize, Serialize};

use crate::{Expression, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logical {
	pub left: Box<Expression>,
	pub op: Node<LogicalOperator>,
	pub right: Box<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperator {
	And,
	Or,
}
