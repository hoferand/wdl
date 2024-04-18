use ast::{Identifier, ScopedIdentifier, Source};

#[derive(Debug, Clone)]
pub struct FunctionId {
	pub id: Identifier,
	pub scope: Vec<Identifier>,
}

impl From<Identifier> for FunctionId {
	fn from(value: Identifier) -> Self {
		Self {
			id: value,
			scope: Vec::new(),
		}
	}
}

impl<S: Source> From<ScopedIdentifier<S>> for FunctionId {
	fn from(value: ScopedIdentifier<S>) -> Self {
		Self {
			id: value.id.val,
			scope: value.scope.into_iter().map(|n| n.val).collect(),
		}
	}
}

impl std::fmt::Display for FunctionId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}",
			self.scope
				.iter()
				.fold(String::new(), |str, id| str + &id.0 + "::"),
			self.id.0
		)
	}
}
