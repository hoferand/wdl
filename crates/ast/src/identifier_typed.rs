use crate::{Identifier, Node, Type};

pub struct IdentfierTyped {
	pub id: Node<Identifier>,
	pub type_: Node<Type>,
}
