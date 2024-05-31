use serde::{Deserialize, Serialize};

/// Represents an identifier.
///
/// Syntax:  
/// [Unicode Standard Annex #31](http://www.unicode.org/reports/tr31/) with XID_Start including `_`
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
