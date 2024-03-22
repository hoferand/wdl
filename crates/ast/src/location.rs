use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
	pub line: usize,   // TODO: should be u64
	pub column: usize, // TODO: should be u64
}
