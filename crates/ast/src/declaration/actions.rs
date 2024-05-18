use serde::{Deserialize, Serialize};

use crate::{Block, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Actions {
	pub block: Node<Block>,
}
