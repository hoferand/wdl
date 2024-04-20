use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Copy)]
pub struct Location {
	pub line: usize,
	pub column: usize,
}
