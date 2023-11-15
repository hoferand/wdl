use crate::{Expression, GlobalDescription, IdentifierFull, Node, Type};

pub struct GlobalDeclaration {
	pub id: Node<IdentifierFull>,
	pub type_: Node<Type>,
	pub value: Option<Expression>, // TODO: improve one of value/description must be present
	pub description: Option<Node<GlobalDescription>>,
}
