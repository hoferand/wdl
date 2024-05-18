use serde::{Deserialize, Serialize};

use crate::{Expression, Identifier, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Let {
	pub id: Node<Identifier>,
	pub value: Expression,
}
