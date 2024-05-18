use serde::{Deserialize, Serialize};

use crate::{Node, Span};

pub mod actions;
pub use actions::*;
pub mod function;
pub use function::*;
pub mod global;
pub use global::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
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
