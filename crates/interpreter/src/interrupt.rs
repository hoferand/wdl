use crate::Value;

#[derive(Debug)]
pub enum Interrupt {
	None,
	Continue,
	Break,
	Return(Value),
}

impl Interrupt {
	pub fn is_none(&self) -> bool {
		matches!(self, Self::None)
	}

	pub fn get_type(&self) -> String {
		match self {
			Self::None => "none",
			Self::Continue => "continue",
			Self::Break => "break",
			Self::Return(_) => "return",
		}
		.to_owned()
	}
}
