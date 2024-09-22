use crate::{Expression, Identifier, Node};

/// Represents a local variable declaration.
///
/// Syntax:  
/// `let` [`Identifier`] `=` [`Expression`] `;`
#[derive(Debug, Clone)]
pub struct Let {
	pub id: Node<Identifier>,
	pub value: Expression,
}
