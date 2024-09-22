use crate::{Expression, Identifier, Node};

/// Represents a function call.
///
/// Syntax:  
/// [`Expression`] `(` ( ( [`Identifier`] `:` )? [`Expression`] `,` )* `)`
#[derive(Debug, Clone)]
pub struct Call {
	pub function: Box<Expression>,
	pub args: Vec<Node<Argument>>,
}

#[derive(Debug, Clone)]
pub struct Argument {
	pub id: Option<Node<Identifier>>,
	pub val: Expression,
}
