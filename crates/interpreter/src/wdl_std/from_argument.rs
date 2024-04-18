use std::collections::HashMap;

use crate::{ChannelId, Error, ErrorKind, FunctionId, Value, ValueType};

use super::{Arg, ArgType, ArgumentValue};

pub(crate) trait FromArgument: Sized {
	fn from_arg(ctx: ArgumentValue) -> Result<Self, Error>;
}

impl FromArgument for Arg<Value> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		Ok(Arg::new(arg.idx, arg.span, arg.val))
	}
}

impl<T: FromArgument> FromArgument for Arg<Vec<T>> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::Array(arr) = arg.val {
			let mut vec = Vec::new();

			for val in arr {
				vec.push(T::from_arg(ArgumentValue {
					idx: arg.idx,
					span: arg.span.clone(),
					val,
				})?);
			}

			Ok(Arg::new(arg.idx, arg.span, vec))
		} else {
			Err(invalid_type(ValueType::Array, arg))
		}
	}
}

impl<T: FromArgument> FromArgument for Arg<HashMap<String, T>> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::Object(obj) = arg.val {
			let mut map = HashMap::new();

			for (id, val) in obj {
				map.insert(
					id,
					T::from_arg(ArgumentValue {
						idx: arg.idx,
						span: arg.span.clone(),
						val,
					})?,
				);
			}

			Ok(Arg::new(arg.idx, arg.span, map))
		} else {
			Err(invalid_type(ValueType::Object, arg))
		}
	}
}

impl FromArgument for Arg<bool> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::Bool(val) = arg.val {
			Ok(Arg::new(arg.idx, arg.span, val))
		} else {
			Err(invalid_type(ValueType::Bool, arg))
		}
	}
}

impl FromArgument for Arg<f64> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::Number(val) = arg.val {
			Ok(Arg::new(arg.idx, arg.span, val))
		} else {
			Err(invalid_type(ValueType::Number, arg))
		}
	}
}

impl FromArgument for Arg<String> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::String(val) = arg.val {
			Ok(Arg::new(arg.idx, arg.span, val))
		} else {
			Err(invalid_type(ValueType::String, arg))
		}
	}
}

impl FromArgument for Arg<FunctionId> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::Function(val) = arg.val {
			Ok(Arg::new(arg.idx, arg.span, val))
		} else {
			Err(invalid_type(ValueType::Function, arg))
		}
	}
}

// TODO: implement for FunctionValue

impl FromArgument for Arg<ChannelId> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::Channel(val) = arg.val {
			Ok(Arg::new(arg.idx, arg.span, val))
		} else {
			Err(invalid_type(ValueType::Channel, arg))
		}
	}
}

// TODO: implement for Channel

impl<T> FromArgument for T
where
	T: for<'de> ArgType<'de>,
{
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		let json_val = match serde_json::to_value(arg.val) {
			Ok(val) => val,
			Err(err) => {
				return Err(Error {
					kind: ErrorKind::Fatal(err.to_string()),
					src: Some(arg.span),
				});
			}
		};

		let rust_val = match serde_json::from_value(json_val) {
			Ok(val) => val,
			Err(err) => {
				return Err(Error {
					kind: ErrorKind::InvalidType {
						msg: err.to_string(),
					},
					src: Some(arg.span),
				});
			}
		};
		Ok(rust_val)
	}
}

fn invalid_type(expected: ValueType, got: ArgumentValue) -> Error {
	Error {
		kind: ErrorKind::InvalidType {
			msg: format!(
				"Expected `{}`, given `{}` for argument {}",
				expected,
				got.val.get_type(),
				got.idx
			),
		},
		src: Some(got.span),
	}
}
