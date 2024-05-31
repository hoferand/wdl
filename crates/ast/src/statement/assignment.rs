use crate::{Expression, Identifier, Node};

/// Represents a variable assignment.
///
/// Syntax:  
/// _Identifier_ `=` _Expression_ `;`
#[derive(Debug, Clone)]
pub struct Assignment {
	pub id: Node<Identifier>,
	pub value: Expression,
}
