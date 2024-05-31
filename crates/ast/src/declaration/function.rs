use crate::{Block, Identifier, Node};

/// Represents a function declaration.
///
/// Syntax:  
/// `function` _Identifier_ `(` ( _Identifier_ `,` )* `)` `{`
///     _Statement_*
/// `}`
#[derive(Debug, Clone)]
pub struct Function {
	pub id: Node<Identifier>,
	pub function: Node<FunctionBody>,
}

#[derive(Debug, Clone)]
pub struct FunctionBody {
	pub parameters: Vec<Node<FormalParameter>>,
	pub body: Node<Block>,
}

#[derive(Debug, Clone)]
pub struct FormalParameter {
	pub id: Node<Identifier>,
}
