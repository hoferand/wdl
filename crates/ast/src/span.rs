use serde::{Deserialize, Serialize};

use crate::Location;

/// column of end is not inclusive
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Span {
	pub start: Location,
	pub end: Location,
}
