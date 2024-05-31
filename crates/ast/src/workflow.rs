use crate::{Actions, Function, Global, Node};

/// Represents a workflow.
///
/// Syntax:  
/// ( _Global_ | _Function_ )* _Actions_ ( _Global_ | _Function_ )*
#[derive(Debug, Clone)]
pub struct Workflow {
	pub globals: Vec<Node<Global>>,
	pub actions: Node<Actions>,
	pub functions: Vec<Node<Function>>,
}
