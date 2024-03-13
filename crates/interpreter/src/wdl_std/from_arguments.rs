use crate::{Error, Value};

use super::Arguments;

pub(crate) trait FromArguments: Sized {
	fn from_args(args: &mut Arguments) -> Result<Self, Error>;
}

impl FromArguments for Value {
	fn from_args(args: &mut Arguments) -> Result<Self, Error> {
		expect(Option::<Self>::from_args(args)?, args)
	}
}

impl FromArguments for Option<Value> {
	fn from_args(args: &mut Arguments) -> Result<Self, Error> {
		Ok(args.args.next().map(|val| val.val))
	}
}

impl FromArguments for bool {
	fn from_args(args: &mut Arguments) -> Result<Self, Error> {
		expect(Option::<Self>::from_args(args)?, args)
	}
}

impl FromArguments for Option<bool> {
	fn from_args(args: &mut Arguments) -> Result<Self, Error> {
		if let Some(arg) = args.args.next() {
			if let Value::Bool(val) = arg.val {
				Ok(Some(val))
			} else {
				Err(Error::InvalidType {
					msg: format!(
						"expected `bool`, given `{}` for argument {}",
						arg.val.get_type(),
						arg.idx
					),
					span: arg.span,
				})
			}
		} else {
			Ok(None)
		}
	}
}

impl FromArguments for f64 {
	fn from_args(args: &mut Arguments) -> Result<Self, Error> {
		expect(Option::<Self>::from_args(args)?, args)
	}
}

impl FromArguments for Option<f64> {
	fn from_args(args: &mut Arguments) -> Result<Self, Error> {
		if let Some(arg) = args.args.next() {
			if let Value::Number(val) = arg.val {
				Ok(Some(val))
			} else {
				Err(Error::InvalidType {
					msg: format!(
						"expected `number`, given `{}` for argument {}",
						arg.val.get_type(),
						arg.idx
					),
					span: arg.span,
				})
			}
		} else {
			Ok(None)
		}
	}
}

impl FromArguments for String {
	fn from_args(args: &mut Arguments) -> Result<Self, Error> {
		expect(Option::<Self>::from_args(args)?, args)
	}
}

impl FromArguments for Option<String> {
	fn from_args(args: &mut Arguments) -> Result<Self, Error> {
		if let Some(arg) = args.args.next() {
			if let Value::String(val) = arg.val {
				Ok(Some(val))
			} else {
				Err(Error::InvalidType {
					msg: format!(
						"expected `string`, given `{}` for argument {}",
						arg.val.get_type(),
						arg.idx
					),
					span: arg.span,
				})
			}
		} else {
			Ok(None)
		}
	}
}

fn expect<T>(val: Option<T>, args: &Arguments) -> Result<T, Error> {
	if let Some(val) = val {
		Ok(val)
	} else {
		Err(Error::TooFewArguments {
			span: args.fn_span.clone(),
		})
	}
}
