use serde::{Deserialize, Serialize};

use crate::{Block, Expression, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct If {
	pub condition: Expression,
	pub then: Node<Block>,
	pub else_: Option<Box<Node<Else>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Else {
	Else(Node<Block>),
	ElseIf(Node<If>),
}
