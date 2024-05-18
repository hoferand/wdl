use serde::{Deserialize, Serialize};

use crate::{Expression, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Logic {
	pub left: Box<Expression>,
	pub op: Node<LogicOperator>,
	pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicOperator {
	And,
	Or,
}
