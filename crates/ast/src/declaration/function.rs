use crate::{Block, Identifier, Node};

/// Represents a function declaration.
///
/// Syntax:  
/// `function` [`Identifier`] `(` ( [`Identifier`] `,` )* `)` [`Block`]
#[derive(Debug, Clone)]
pub struct Function {
	pub id: Node<Identifier>,
	pub params: Vec<Node<Identifier>>,
	pub body: Node<Block>,
}
