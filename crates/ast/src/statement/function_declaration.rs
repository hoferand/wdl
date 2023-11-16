use crate::{Function, Identifier, Node};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
	pub id: Node<Identifier>,
	pub function: Node<Function>,
}
