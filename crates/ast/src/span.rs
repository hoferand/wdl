use serde::{Deserialize, Serialize};

use crate::Location;

/// Represents a span inside the source code.
///
/// CAUTION: `column` of `end` is exclusive.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Copy)]
#[serde(tag = "type")]
pub struct Span {
	pub start: Location,
	pub end: Location,
}
