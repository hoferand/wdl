use crate::{Block, Expression, Node};

pub struct If {
	pub condition: Expression,
	pub then: Node<Block>,
	pub else_: Box<Node<Else>>,
}

pub enum Else {
	Else(Node<Block>),
	ElseIf(Node<If>),
}
