use std::vec::IntoIter;

use crate::{Error, Value};

pub(crate) trait FromArguments: Sized {
	fn from_args(args: &mut IntoIter<Value>) -> Result<Self, Error>;
}

impl FromArguments for Value {
	fn from_args(args: &mut IntoIter<Value>) -> Result<Self, Error> {
		if let Some(arg) = args.next() {
			Ok(arg)
		} else {
			// TODO: improve error
			Err(Error::Fatal("Too few arguments given".to_owned()))
		}
	}
}

impl FromArguments for Option<Value> {
	fn from_args(args: &mut IntoIter<Value>) -> Result<Self, Error> {
		Ok(args.next())
	}
}

impl FromArguments for bool {
	fn from_args(args: &mut IntoIter<Value>) -> Result<Self, Error> {
		if let Some(Value::Bool(arg)) = args.next() {
			Ok(arg)
		} else {
			// TODO: improve error
			Err(Error::Fatal("Error with argument".to_owned()))
		}
	}
}

impl FromArguments for Option<bool> {
	fn from_args(args: &mut IntoIter<Value>) -> Result<Self, Error> {
		if let Some(arg) = args.next() {
			if let Value::Bool(arg) = arg {
				Ok(Some(arg))
			} else {
				// TODO: improve error
				Err(Error::Fatal("Error with argument".to_owned()))
			}
		} else {
			Ok(None)
		}
	}
}

impl FromArguments for f64 {
	fn from_args(args: &mut IntoIter<Value>) -> Result<Self, Error> {
		if let Some(Value::Number(arg)) = args.next() {
			Ok(arg)
		} else {
			// TODO: improve error
			Err(Error::Fatal("Error with argument".to_owned()))
		}
	}
}

impl FromArguments for Option<f64> {
	fn from_args(args: &mut IntoIter<Value>) -> Result<Self, Error> {
		if let Some(arg) = args.next() {
			if let Value::Number(arg) = arg {
				Ok(Some(arg))
			} else {
				// TODO: improve error
				Err(Error::Fatal("Error with argument".to_owned()))
			}
		} else {
			Ok(None)
		}
	}
}

impl FromArguments for String {
	fn from_args(args: &mut IntoIter<Value>) -> Result<Self, Error> {
		if let Some(Value::String(arg)) = args.next() {
			Ok(arg)
		} else {
			// TODO: improve error
			Err(Error::Fatal("Error with argument".to_owned()))
		}
	}
}

impl FromArguments for Option<String> {
	fn from_args(args: &mut IntoIter<Value>) -> Result<Self, Error> {
		if let Some(arg) = args.next() {
			if let Value::String(arg) = arg {
				Ok(Some(arg))
			} else {
				// TODO: improve error
				Err(Error::Fatal("Error with argument".to_owned()))
			}
		} else {
			Ok(None)
		}
	}
}
