use serde::{Deserialize, Serialize};

use crate::{Expression, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unary {
	pub op: Node<UnaryOperator>,
	pub right: Box<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOperator {
	Negate,
	Flip,
}
