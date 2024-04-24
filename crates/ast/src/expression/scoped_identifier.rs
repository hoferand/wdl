use serde::{Deserialize, Serialize};

use crate::{Identifier, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ScopedIdentifier<S: Source> {
	pub id: Node<S, Identifier>,
	pub scope: Vec<Node<S, Identifier>>,
}

impl<S: Source> std::fmt::Display for ScopedIdentifier<S> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}",
			self.scope
				.iter()
				.fold(String::new(), |str, id| str + &id.val.id + "::"),
			self.id.val.id
		)
	}
}
