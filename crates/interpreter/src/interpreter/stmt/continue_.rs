use crate::{Error, Interrupt};

pub fn interpret_continue() -> Result<Interrupt, Error> {
	Ok(Interrupt::Continue)
}
