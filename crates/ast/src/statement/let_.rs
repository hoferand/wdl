use crate::{Expression, Identifier, Node};

/// Represents a local variable declaration.
///
/// Syntax:  
/// `let` _Identifier_ `=` _Expression_ `;`
#[derive(Debug, Clone)]
pub struct Let {
	pub id: Node<Identifier>,
	pub value: Expression,
}
