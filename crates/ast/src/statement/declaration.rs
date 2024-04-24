use serde::{Deserialize, Serialize};

use crate::{Actions, FunctionDeclaration, GlobalDeclaration, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Declaration<S: Source> {
	GlobalDeclaration(Node<S, GlobalDeclaration<S>>),
	Actions(Node<S, Actions<S>>),
	FunctionDeclaration(Node<S, FunctionDeclaration<S>>),
}

impl<S: Source> Declaration<S> {
	pub fn get_src(&self) -> &S {
		match self {
			Declaration::GlobalDeclaration(stmt) => &stmt.src,
			Declaration::Actions(stmt) => &stmt.src,
			Declaration::FunctionDeclaration(stmt) => &stmt.src,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Declaration::GlobalDeclaration(_) => "global declaration",
			Declaration::Actions(_) => "actions",
			Declaration::FunctionDeclaration(_) => "function declaration",
		}
		.to_owned()
	}
}
