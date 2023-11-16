use crate::Location;

/// column of end is not inclusive
#[derive(Debug, Default)]
pub struct Span {
	pub start: Location,
	pub end: Location,
}
