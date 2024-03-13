use serde::{Deserialize, Serialize};

use crate::{Block, Identifier, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
	pub parameter: Vec<Node<Parameter>>,
	pub body: Node<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
	pub id: Node<Identifier>,
}
