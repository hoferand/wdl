use crate::{Identifier, Node};

#[derive(Debug, Clone)]
pub struct Member {
	pub member: Node<Identifier>,
}
