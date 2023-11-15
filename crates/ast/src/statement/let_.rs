use crate::{Expression, IdentifierFull, Node, Type};

pub struct Let {
	pub id: Node<IdentifierFull>,
	pub type_: Node<Type>,
	pub value: Expression,
}
