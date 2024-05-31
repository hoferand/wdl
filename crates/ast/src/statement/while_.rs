use crate::{Block, Expression, Node};

/// Represents a while statement.
///
/// Syntax:  
/// `while` _Expression_ `{` _Statement_* `}`
#[derive(Debug, Clone)]
pub struct While {
	pub condition: Expression,
	pub do_: Node<Block>,
}
