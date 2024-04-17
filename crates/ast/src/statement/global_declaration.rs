use serde::{Deserialize, Serialize};

use crate::{Expression, Identifier, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalDeclaration<S: Source> {
	pub id: Node<S, Identifier>,
	pub value: Option<Expression<S>>,
}
