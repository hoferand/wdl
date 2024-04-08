use serde::Deserialize;

use crate::{channel::Channel, Error, Value};

use super::Arguments;

pub(crate) trait FromArguments: Sized {
	fn from_args(args: &mut Arguments) -> Result<Self, Error>;
}

impl<T> FromArguments for T
where
	T: for<'de> Deserialize<'de>,
{
	fn from_args(args: &mut Arguments) -> Result<Self, Error> {
		if let Some(arg) = args.args.next() {
			let json_val = match serde_json::to_value(arg.val) {
				Ok(val) => val,
				Err(err) => {
					return Err(Error::InvalidType {
						msg: err.to_string(),
						span: arg.span,
					});
				}
			};

			let rust_val = match serde_json::from_value(json_val) {
				Ok(val) => val,
				Err(err) => {
					return Err(Error::InvalidType {
						msg: err.to_string(),
						span: arg.span,
					});
				}
			};
			Ok(rust_val)
		} else {
			Err(Error::TooFewArguments {
				span: args.fn_span.clone(),
			})
		}
	}
}

impl FromArguments for Channel {
	fn from_args(args: &mut Arguments) -> Result<Self, Error> {
		if let Some(arg) = args.args.next() {
			if let Value::Channel(ch) = arg.val {
				Ok(ch)
			} else {
				Err(Error::InvalidType {
					msg: format!("Expected channel but got `{}`", arg.val.get_type()),
					span: arg.span,
				})
			}
		} else {
			Err(Error::TooFewArguments {
				span: args.fn_span.clone(),
			})
		}
	}
}
