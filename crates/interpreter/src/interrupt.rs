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
		matches!(self, Interrupt::None)
	}

	pub fn get_type(&self) -> String {
		match self {
			Interrupt::None => "none",
			Interrupt::Continue => "continue",
			Interrupt::Break => "break",
			Interrupt::Return(_) => "return",
		}
		.to_owned()
	}
}
