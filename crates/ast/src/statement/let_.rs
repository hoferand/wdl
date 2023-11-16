use crate::{Expression, IdentifierTyped, Node};

#[derive(Debug, Clone)]
pub struct Let {
	pub id: Node<IdentifierTyped>,
	pub value: Expression,
}
