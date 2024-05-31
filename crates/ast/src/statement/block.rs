use crate::Statement;

/// Represents a code block.
///
/// Syntax:  
/// `{` _Statement_* `}`
#[derive(Debug, Clone)]
pub struct Block {
	pub stmts: Vec<Statement>,
}
