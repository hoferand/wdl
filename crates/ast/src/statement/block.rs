use crate::Statement;

/// Represents a code block.
///
/// Syntax:  
/// `{` [`Statement`]* `}`
#[derive(Debug, Clone)]
pub struct Block {
	pub stmts: Vec<Statement>,
}
