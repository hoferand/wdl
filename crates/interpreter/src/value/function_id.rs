use serde::{Deserialize, Serialize};

use ast::{Identifier, Variable};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "wdl_type")]
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

impl From<Variable> for FunctionId {
	fn from(value: Variable) -> Self {
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
				.fold(String::new(), |str, id| str + &id.id + "::"),
			self.id.id
		)
	}
}
