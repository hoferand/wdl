use crate::Expression;

/// Represents an array value.
///
/// Syntax:  
/// `[` ( [`Expression`] `,` )* `]`
#[derive(Debug, Clone)]
pub struct Array {
	pub values: Vec<Expression>,
}
