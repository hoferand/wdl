use crate::{Actions, Function, Global, Node};

/// Represents a workflow.
///
/// Syntax:  
/// ( [`Global`] | [`Function`] )* [`Actions`] ( [`Global`] | [`Function`] )*
#[derive(Debug, Clone)]
pub struct Workflow {
	pub globals: Vec<Node<Global>>,
	pub actions: Node<Actions>,
	pub functions: Vec<Node<Function>>,
}
