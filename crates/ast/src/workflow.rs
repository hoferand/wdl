use serde::{Deserialize, Serialize};

use crate::{Actions, FunctionDeclaration, GlobalDeclaration, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow<S: Source> {
	pub globals: Vec<Node<S, GlobalDeclaration<S>>>,
	pub actions: Node<S, Actions<S>>,
	pub functions: Vec<Node<S, FunctionDeclaration<S>>>,
}
