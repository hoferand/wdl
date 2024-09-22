use crate::{Block, Expression, Node};

/// Represents a while statement.
///
/// Syntax:  
/// `while` [`Expression`] [`Block`]
#[derive(Debug, Clone)]
pub struct While {
	pub condition: Expression,
	pub do_: Node<Block>,
}
