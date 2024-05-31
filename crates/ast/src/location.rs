use serde::{Deserialize, Serialize};

/// Represents a location inside the source code.
///
/// CAUTION: `line` and `column` are 0-based.
#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Location {
	pub line: usize,
	pub column: usize,
}
