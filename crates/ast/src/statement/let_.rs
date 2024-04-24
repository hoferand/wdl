use serde::{Deserialize, Serialize};

use crate::{Expression, Identifier, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Let<S: Source> {
	pub id: Node<S, Identifier>,
	pub value: Expression<S>,
}
