use serde::{Deserialize, Serialize};

use crate::{Expression, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logical<S: Source> {
	pub left: Box<Expression<S>>,
	pub op: Node<S, LogicalOperator>,
	pub right: Box<Expression<S>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperator {
	And,
	Or,
}
