use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
	pub line: usize,
	pub column: usize,
}
