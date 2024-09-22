use crate::Expression;

/// Represents a spawn expression.
///
/// Syntax:  
/// `spawn` [`Expression`]
#[derive(Debug, Clone)]
pub struct Spawn {
	pub expr: Box<Expression>,
}
