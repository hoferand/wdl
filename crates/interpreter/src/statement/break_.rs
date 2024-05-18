use crate::{Error, Interrupt};

pub fn interpret_break() -> Result<Interrupt, Error> {
	Ok(Interrupt::Break)
}
