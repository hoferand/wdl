use crate::{FunctionDeclaration, Import, Node};

pub struct Module {
	pub imports: Vec<Node<Import>>,
	pub functions: Vec<Node<FunctionDeclaration>>,
}
