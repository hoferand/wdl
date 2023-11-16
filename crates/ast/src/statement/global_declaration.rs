use crate::{Expression, GlobalDescription, IdentifierTyped, Node};

#[derive(Debug, Clone)]
pub struct GlobalDeclaration {
	pub id: Node<IdentifierTyped>,
	pub value: Option<Expression>, // TODO: improve one of value/description must be present
	pub description: Option<Node<GlobalDescription>>,
}
