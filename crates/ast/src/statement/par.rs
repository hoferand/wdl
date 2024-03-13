use serde::{Deserialize, Serialize};

use crate::{Block, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Par {
	pub blocks: Vec<Node<Block>>,
}
