use serde::{Deserialize, Serialize};

use crate::{Expression, Identifier, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Call {
	pub function: Box<Expression>,
	pub args: Vec<Node<Argument>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Argument {
	pub id: Option<Node<Identifier>>,
	pub val: Expression,
}
