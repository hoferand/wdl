use crate::{Block, Expression, Node};

#[derive(Debug, Clone)]
pub struct While {
	pub condition: Expression,
	pub do_: Node<Block>,
}
