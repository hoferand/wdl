use serde::{Deserialize, Serialize};

use crate::{Block, Identifier, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Function {
	pub id: Node<Identifier>,
	pub function: Node<FunctionBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct FunctionBody {
	pub parameters: Vec<Node<FormalParameter>>,
	pub body: Node<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct FormalParameter {
	pub id: Node<Identifier>,
}
