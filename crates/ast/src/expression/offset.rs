use crate::Expression;

/// Represents an offset expression.
///
/// Syntax:  
/// _Expression_ `[` _Expression_ `]`
#[derive(Debug, Clone)]
pub struct Offset {
	pub value: Box<Expression>,
	pub offset: Box<Expression>,
}
