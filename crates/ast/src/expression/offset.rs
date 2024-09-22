use crate::Expression;

/// Represents an offset expression.
///
/// Syntax:  
/// [`Expression`] `[` [`Expression`] `]`
#[derive(Debug, Clone)]
pub struct Offset {
	pub value: Box<Expression>,
	pub offset: Box<Expression>,
}
