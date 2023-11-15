use crate::{Block, Expression, Node};

pub struct While {
	pub condition: Expression,
	pub do_: Node<Block>,
}
