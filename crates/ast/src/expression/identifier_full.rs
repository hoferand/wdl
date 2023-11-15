use crate::{Identifier, Node};

pub struct IdentifierFull {
	pub id: Node<Identifier>,
	pub module: Vec<Node<IdentifierFull>>,
}
