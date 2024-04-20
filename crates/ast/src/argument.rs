use serde::{Deserialize, Serialize};

use crate::{Expression, Identifier, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argument<S: Source> {
	pub id: Option<Node<S, Identifier>>,
	pub val: Expression<S>,
}
