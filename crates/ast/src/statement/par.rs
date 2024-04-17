use serde::{Deserialize, Serialize};

use crate::{Block, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Par<S: Source> {
	pub blocks: Vec<Node<S, Block<S>>>,
}
