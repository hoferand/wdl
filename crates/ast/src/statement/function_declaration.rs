use serde::{Deserialize, Serialize};

use crate::{Function, Identifier, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDeclaration<S: Source> {
	pub id: Node<S, Identifier>,
	pub function: Node<S, Function<S>>,
}
