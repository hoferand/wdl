use serde::{Deserialize, Serialize};

use crate::{Actions, FunctionDeclaration, GlobalDeclaration, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
	pub globals: Vec<Node<GlobalDeclaration>>,
	pub actions: Node<Actions>,
	pub functions: Vec<Node<FunctionDeclaration>>,
}
