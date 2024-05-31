use crate::Expression;

/// Represents an array value.
///
/// Syntax:  
/// `[` ( _Expression_ `,` )* `]`
#[derive(Debug, Clone)]
pub struct Array {
	pub values: Vec<Expression>,
}
