use crate::{Block, Expression, Node};

/// Represents an if-else statement.
///
/// Syntax:  
/// `if` _Expression_ `{` _Statement_* `}` ( `else` ( _If_ | `{` _Statement_* `}` ) )?
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
