use crate::{Function, Identifier, Node};

pub struct FunctionDeclaration {
	pub id: Node<Identifier>,
	pub function: Node<Function>,
}
