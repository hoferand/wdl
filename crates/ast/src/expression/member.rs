use serde::{Deserialize, Serialize};

use crate::{Expression, Identifier, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member<S: Source> {
	pub object: Box<Expression<S>>,
	pub member: Node<S, Identifier>,
}
