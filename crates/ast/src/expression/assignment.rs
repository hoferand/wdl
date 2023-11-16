use crate::{Expression, IdentifierFull, Node};

#[derive(Debug, Clone)]
pub struct Assignment {
	pub id: Node<IdentifierFull>,
	pub value: Box<Expression>,
}
