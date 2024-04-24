use serde::{Deserialize, Serialize};

use crate::{Expression, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Unary<S: Source> {
	pub op: Node<S, UnaryOperator>,
	pub right: Box<Expression<S>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOperator {
	Negate,
	Flip,
	Receive,
}
