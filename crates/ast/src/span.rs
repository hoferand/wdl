use crate::Location;

#[derive(Default)]
pub struct Span {
	pub start: Location,
	pub end: Location,
}
