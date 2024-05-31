use crate::{Block, Node};

/// Represents the entry point of a workflow.
///
/// Syntax:  
/// `actions` `{`
///     _Statement_*
/// `}`
#[derive(Debug, Clone)]
pub struct Actions {
	pub block: Node<Block>,
}
