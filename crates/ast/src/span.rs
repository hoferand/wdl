use crate::Location;

/// column of end is not inclusive
#[derive(Debug, Clone, Default)]
pub struct Span {
	pub start: Location,
	pub end: Location,
}
