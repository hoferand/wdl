use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Copy)]
#[serde(tag = "type")]
pub struct Location {
	pub line: usize,
	pub column: usize,
}
