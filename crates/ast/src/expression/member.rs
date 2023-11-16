use crate::{IdentifierFull, Node};

#[derive(Debug, Clone)]
pub struct Member {
	pub member: Node<IdentifierFull>,
}
