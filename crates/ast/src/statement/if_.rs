use serde::{Deserialize, Serialize};

use crate::{Block, Expression, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct If<S: Source> {
	pub condition: Expression<S>,
	pub then: Node<S, Block<S>>,
	pub else_: Option<Box<Node<S, Else<S>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Else<S: Source> {
	Else(Node<S, Block<S>>),
	ElseIf(Node<S, If<S>>),
}
