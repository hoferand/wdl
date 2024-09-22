use crate::{Expression, Identifier, Node};

/// Represents a global variable declaration.
///
/// Syntax:  
/// `global` [`Identifier`] `=` [`Expression`] `;`
#[derive(Debug, Clone)]
pub struct Global {
	pub id: Node<Identifier>,
	pub value: Expression,
}
