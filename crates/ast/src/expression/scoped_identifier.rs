use serde::{Deserialize, Serialize};

use crate::{Identifier, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopedIdentifier {
	pub id: Node<Identifier>,
	pub scope: Vec<Node<Identifier>>,
}

impl std::fmt::Display for ScopedIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}",
			self.scope
				.iter()
				.fold(String::new(), |str, id| str + &id.val.0 + "::"),
			self.id.val.0
		)
	}
}
