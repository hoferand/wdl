use crate::Expression;

/// Represents a group expression.
///
/// Syntax:  
/// `(` [`Expression`] `)`
#[derive(Debug, Clone)]
pub struct Group {
	pub expression: Box<Expression>,
}
