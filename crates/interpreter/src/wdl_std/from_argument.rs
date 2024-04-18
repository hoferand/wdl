use std::collections::HashMap;

use crate::{ChannelId, Error, ErrorKind, FunctionId, Value};

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
			Err(Error {
				kind: ErrorKind::InvalidType {
					msg: format!(
						"expected `array`, given `{}` for argument {}",
						arg.val.get_type(),
						arg.idx
					),
				},
				src: Some(arg.span),
			})
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
			Err(Error {
				kind: ErrorKind::InvalidType {
					msg: format!(
						"expected `object`, given `{}` for argument {}",
						arg.val.get_type(),
						arg.idx
					),
				},
				src: Some(arg.span),
			})
		}
	}
}

impl FromArgument for Arg<bool> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::Bool(val) = arg.val {
			Ok(Arg::new(arg.idx, arg.span, val))
		} else {
			Err(Error {
				kind: ErrorKind::InvalidType {
					msg: format!(
						"expected `bool`, given `{}` for argument {}",
						arg.val.get_type(),
						arg.idx
					),
				},
				src: Some(arg.span),
			})
		}
	}
}

impl FromArgument for Arg<f64> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::Number(val) = arg.val {
			Ok(Arg::new(arg.idx, arg.span, val))
		} else {
			Err(Error {
				kind: ErrorKind::InvalidType {
					msg: format!(
						"expected `number`, given `{}` for argument {}",
						arg.val.get_type(),
						arg.idx
					),
				},
				src: Some(arg.span),
			})
		}
	}
}

impl FromArgument for Arg<String> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::String(val) = arg.val {
			Ok(Arg::new(arg.idx, arg.span, val))
		} else {
			Err(Error {
				kind: ErrorKind::InvalidType {
					msg: format!(
						"expected `string`, given `{}` for argument {}",
						arg.val.get_type(),
						arg.idx
					),
				},
				src: Some(arg.span),
			})
		}
	}
}

impl FromArgument for Arg<FunctionId> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::Function(val) = arg.val {
			Ok(Arg::new(arg.idx, arg.span, val))
		} else {
			Err(Error {
				kind: ErrorKind::InvalidType {
					msg: format!(
						"expected `function`, given `{}` for argument {}",
						arg.val.get_type(),
						arg.idx
					),
				},
				src: Some(arg.span),
			})
		}
	}
}

// TODO: implement for FunctionValue

impl FromArgument for Arg<ChannelId> {
	fn from_arg(arg: ArgumentValue) -> Result<Self, Error> {
		if let Value::Channel(val) = arg.val {
			Ok(Arg::new(arg.idx, arg.span, val))
		} else {
			Err(Error {
				kind: ErrorKind::InvalidType {
					msg: format!(
						"expected `channel`, given `{}` for argument {}",
						arg.val.get_type(),
						arg.idx
					),
				},
				src: Some(arg.span),
			})
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
