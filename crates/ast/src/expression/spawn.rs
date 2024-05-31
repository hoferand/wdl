use crate::Expression;

/// Represents a spawn expression.
///
/// Syntax:  
/// `spawn` _Expression_
#[derive(Debug, Clone)]
pub struct Spawn {
	pub expr: Box<Expression>,
}
