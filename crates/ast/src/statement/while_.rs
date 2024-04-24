use serde::{Deserialize, Serialize};

use crate::{Block, Expression, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct While<S: Source> {
	pub condition: Expression<S>,
	pub do_: Node<S, Block<S>>,
}
