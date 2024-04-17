use serde::{Deserialize, Serialize};

use crate::{Expression, Identifier, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment<S: Source> {
	pub id: Node<S, Identifier>,
	pub value: Box<Expression<S>>,
}
