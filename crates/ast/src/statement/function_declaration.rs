use serde::{Deserialize, Serialize};

use crate::{Function, Identifier, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDeclaration {
	pub id: Node<Identifier>,
	pub function: Node<Function>,
}
