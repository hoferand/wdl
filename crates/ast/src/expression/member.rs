use crate::{Expression, Identifier, Node};

/// Represents a member expression.
///
/// Syntax:  
/// _Expression_ `.` _Identifier_
#[derive(Debug, Clone)]
pub struct Member {
	pub object: Box<Expression>,
	pub member: Node<Identifier>,
}
