use crate::{Identifier, Node};

/// Represents a variable.
///
/// Syntax:  
/// ( _Identifier_ `::` )* _Identifier_
#[derive(Debug, Clone)]
pub struct Variable {
	pub id: Node<Identifier>,
	pub scope: Vec<Node<Identifier>>,
}

impl std::fmt::Display for Variable {
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
