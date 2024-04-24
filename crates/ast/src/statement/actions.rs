use serde::{Deserialize, Serialize};

use crate::{Block, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Actions<S: Source> {
	pub block: Node<S, Block<S>>,
}
