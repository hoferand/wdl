use serde::{Deserialize, Serialize};

use crate::{Block, Expression, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct While {
	pub condition: Expression,
	pub do_: Node<Block>,
}
