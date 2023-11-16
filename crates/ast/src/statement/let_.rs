use crate::{Expression, Identifier, Node, Type};

#[derive(Debug, Clone)]
pub struct Let {
	pub id: Node<Identifier>,
	pub type_: Node<Type>,
	pub value: Expression,
}
