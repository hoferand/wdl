use std::vec::IntoIter;

use crate::{Error, Value};

pub(crate) trait FromArguments: Sized {
	fn from_args(args: &mut IntoIter<Value>) -> Result<Self, Error>;
}

impl FromArguments for Value {
	fn from_args(args: &mut IntoIter<Value>) -> Result<Self, Error> {
		if let Some(arg) = args.next() {
			Ok(arg.clone())
		} else {
			// TODO: improve error
			Err(Error::Fatal("Too few arguments given!".to_owned()))
		}
	}
}
