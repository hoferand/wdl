use crate::{Expression, Identifier, Node};

/// Represents a global variable declaration.
///
/// Syntax:  
/// `global` _Identifier_ `=` _Expression_ `;`
#[derive(Debug, Clone)]
pub struct Global {
	pub id: Node<Identifier>,
	pub value: Expression,
}
