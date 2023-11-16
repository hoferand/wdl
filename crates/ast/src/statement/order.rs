use crate::{Block, Node};

#[derive(Debug, Clone)]
pub struct Order {
	pub block: Node<Block>,
}
