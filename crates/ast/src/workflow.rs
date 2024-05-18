use serde::{Deserialize, Serialize};

use crate::{Actions, Function, Global, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Workflow {
	pub globals: Vec<Node<Global>>,
	pub actions: Node<Actions>,
	pub functions: Vec<Node<Function>>,
}
