use crate::Expression;

/// Represents a group expression.
///
/// Syntax:  
/// `(` _Expression_ `)`
#[derive(Debug, Clone)]
pub struct Group {
	pub expression: Box<Expression>,
}
