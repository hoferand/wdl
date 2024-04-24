use serde::{Deserialize, Serialize};

use crate::{Location, Source};

/// column of end is not inclusive
#[derive(Debug, Clone, Default, Serialize, Deserialize, Copy)]
#[serde(tag = "type")]
pub struct Span {
	pub start: Location,
	pub end: Location,
}

impl Source for Span {}
