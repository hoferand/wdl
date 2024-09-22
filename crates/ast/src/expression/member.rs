use crate::{Expression, Identifier, Node};

/// Represents a member expression.
///
/// Syntax:  
/// [`Expression`] `.` [`Identifier`]
#[derive(Debug, Clone)]
pub struct Member {
	pub object: Box<Expression>,
	pub member: Node<Identifier>,
}
