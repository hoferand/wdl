use crate::{Expression, Identifier, Node};

#[derive(Debug, Clone)]
pub struct Let {
	pub id: Node<Identifier>,
	//pub type_: Node<Type>,
	pub value: Expression,
}
