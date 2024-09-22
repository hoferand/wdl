use crate::{Expression, Identifier, Node};

/// Represents a variable assignment.
///
/// Syntax:  
/// [`Identifier`] `=` [`Expression`] `;`
#[derive(Debug, Clone)]
pub struct Assignment {
	pub id: Node<Identifier>,
	pub value: Expression,
}
