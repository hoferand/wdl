use crate::{Block, Node};

#[derive(Debug, Clone)]
pub struct Par {
	pub blocks: Vec<Node<Block>>,
}
