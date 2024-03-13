use serde::{Deserialize, Serialize};

use crate::{Expression, Identifier, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
	pub object: Box<Expression>,
	pub member: Node<Identifier>,
}
