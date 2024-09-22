use crate::Expression;

/// Represents a return statement.
///
/// Syntax:  
/// `return` [`Expression`] `;`
#[derive(Debug, Clone)]
pub struct Return {
	pub value: Option<Expression>,
}
