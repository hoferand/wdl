use crate::{Node, Span};

mod actions;
pub use actions::*;
mod function;
pub use function::*;
mod global;
pub use global::*;

/// Represents a declaration on the outermost scope.
#[derive(Debug, Clone)]
pub enum Declaration {
	Actions(Node<Actions>),
	FunctionDeclaration(Node<Function>),
	GlobalDeclaration(Node<Global>),
}

impl Declaration {
	pub fn get_span(&self) -> &Span {
		match self {
			Self::Actions(stmt) => &stmt.span,
			Self::FunctionDeclaration(stmt) => &stmt.span,
			Self::GlobalDeclaration(stmt) => &stmt.span,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Self::Actions(_) => "actions",
			Self::FunctionDeclaration(_) => "function",
			Self::GlobalDeclaration(_) => "global",
		}
		.to_owned()
	}
}
