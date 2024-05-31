use crate::Expression;

/// Represents a return statement.
///
/// Syntax:  
/// `return` _Expression_ `;`
#[derive(Debug, Clone)]
pub struct Return {
	pub value: Option<Expression>,
}
