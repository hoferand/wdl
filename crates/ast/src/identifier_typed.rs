use crate::{Identifier, Node, Type};

#[derive(Debug, Clone)]
pub struct IdentifierTyped {
	pub id: Node<Identifier>,
	pub type_: Node<Type>,
}
