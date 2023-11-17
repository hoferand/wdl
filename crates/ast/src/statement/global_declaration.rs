use crate::{Expression, GlobalDescription, Identifier, Node};

#[derive(Debug, Clone)]
pub struct GlobalDeclaration {
	pub id: Node<Identifier>,
	//pub type_: Node<Type>,
	pub value: Option<Expression>, // TODO: improve one of value/description must be present
	pub description: Option<Node<GlobalDescription>>,
}
