use crate::{Block, Expression, Node};

/// Represents an if-else statement.
///
/// Syntax:  
/// `if` [`Expression`] [`Block`] ( `else` ( [`If`] | [`Block`] ) )?
#[derive(Debug, Clone)]
pub struct If {
	pub condition: Expression,
	pub then: Node<Block>,
	pub else_: Option<Box<Node<Else>>>,
}

#[derive(Debug, Clone)]
pub enum Else {
	Else(Node<Block>),
	ElseIf(Node<If>),
}
