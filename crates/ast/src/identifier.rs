use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Identifier {
	pub id: String,
}

impl std::fmt::Display for Identifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.id)
	}
}

// necessary for `join()`
impl Borrow<str> for Identifier {
	fn borrow(&self) -> &str {
		&self.id
	}
}
