use crate::{Identifier, Node};

#[derive(Debug, Clone)]
pub struct IdentifierFull {
	pub id: Node<Identifier>,
	pub module: Vec<Node<Identifier>>,
}
