use serde::{Deserialize, Serialize};

use crate::{Expression, GlobalDescription, Identifier, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalDeclaration {
	pub id: Node<Identifier>,
	pub value: Option<Expression>, // TODO: improve one of value/description must be present
	pub description: Option<Node<GlobalDescription>>,
}
