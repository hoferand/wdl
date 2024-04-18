use std::collections::HashMap;

use crate::{ChannelId, Error, FunctionId, Value};

use super::ResultType;

pub(crate) trait IntoResult {
	fn into_result(self) -> Result<Value, Error>;
}

impl<T: IntoResult> IntoResult for Option<T> {
	fn into_result(self) -> Result<Value, Error> {
		match self {
			Some(val) => T::into_result(val),
			None => Ok(Value::Null),
		}
	}
}

impl<T: IntoResult> IntoResult for Result<T, Error> {
	fn into_result(self) -> Result<Value, Error> {
		T::into_result(self?)
	}
}

impl<T: IntoResult> IntoResult for Vec<T> {
	fn into_result(self) -> Result<Value, Error> {
		let mut vec = Vec::new();

		for val in self {
			vec.push(T::into_result(val)?);
		}

		Ok(Value::Array(vec))
	}
}

impl<T: IntoResult> IntoResult for HashMap<String, T> {
	fn into_result(self) -> Result<Value, Error> {
		let mut map = HashMap::new();

		for (id, val) in self {
			map.insert(id, T::into_result(val)?);
		}

		Ok(Value::Object(map))
	}
}

impl IntoResult for Value {
	fn into_result(self) -> Result<Value, Error> {
		Ok(self)
	}
}

impl IntoResult for () {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::Null)
	}
}

impl IntoResult for bool {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::Bool(self))
	}
}

impl IntoResult for f64 {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::Number(self))
	}
}

impl IntoResult for String {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::String(self))
	}
}

impl IntoResult for FunctionId {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::Function(self))
	}
}

impl IntoResult for ChannelId {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::Channel(self))
	}
}

impl<T: ResultType> IntoResult for T {
	fn into_result(self) -> Result<Value, Error> {
		let json_val = match serde_json::to_value(self) {
			Ok(val) => val,
			Err(err) => return Err(Error::fatal(err.to_string())),
		};

		let wdl_val = match serde_json::from_value(json_val) {
			Ok(val) => val,
			Err(err) => return Err(Error::fatal(err.to_string())),
		};
		Ok(wdl_val)
	}
}
