use crate::{ChannelId, Error, ErrorKind, FunctionId, Value, ValueType};

use super::ArgType;

pub trait FromValue: Sized {
	fn from_value(val: Value) -> Result<Option<Self>, Error>;
	fn get_type() -> ValueType;
}

impl FromValue for Value {
	fn from_value(val: Value) -> Result<Option<Self>, Error> {
		Ok(Some(val))
	}

	fn get_type() -> ValueType {
		ValueType::Any
	}
}

impl FromValue for bool {
	fn from_value(val: Value) -> Result<Option<Self>, Error> {
		if let Value::Bool(val) = val {
			Ok(Some(val))
		} else {
			Ok(None)
		}
	}

	fn get_type() -> ValueType {
		ValueType::Bool
	}
}

impl FromValue for f64 {
	fn from_value(val: Value) -> Result<Option<Self>, Error> {
		if let Value::Number(val) = val {
			Ok(Some(val))
		} else {
			Ok(None)
		}
	}

	fn get_type() -> ValueType {
		ValueType::Number
	}
}

impl FromValue for String {
	fn from_value(val: Value) -> Result<Option<Self>, Error> {
		if let Value::String(val) = val {
			Ok(Some(val))
		} else {
			Ok(None)
		}
	}

	fn get_type() -> ValueType {
		ValueType::String
	}
}

impl FromValue for FunctionId {
	fn from_value(val: Value) -> Result<Option<Self>, Error> {
		if let Value::Function(val) = val {
			Ok(Some(val))
		} else {
			Ok(None)
		}
	}

	fn get_type() -> ValueType {
		ValueType::Function
	}
}

// TODO: implement for FunctionValue

impl FromValue for ChannelId {
	fn from_value(val: Value) -> Result<Option<Self>, Error> {
		if let Value::Channel(val) = val {
			Ok(Some(val))
		} else {
			Ok(None)
		}
	}

	fn get_type() -> ValueType {
		ValueType::Channel
	}
}

// TODO: implement for Channel

impl<T> FromValue for T
where
	T: for<'de> ArgType<'de>,
{
	fn from_value(val: Value) -> Result<Option<Self>, Error> {
		// TODO: do not abuse json for that

		if val == Value::Null {
			return Ok(None);
		}

		let json_val = match serde_json::to_value(val) {
			Ok(val) => val,
			Err(err) => {
				return Err(Error::fatal(err.to_string()));
			}
		};

		let rust_val = match serde_json::from_value(json_val) {
			Ok(val) => val,
			Err(err) => {
				return Err(Error {
					kind: ErrorKind::InvalidType {
						msg: err.to_string(),
					},
					span: None,
				});
			}
		};
		Ok(Some(rust_val))
	}

	fn get_type() -> ValueType {
		ValueType::Object
	}
}
