use crate::{Error, Value};

pub(crate) trait IntoResult {
	fn into_result(self) -> Result<Value, Error>;
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

impl IntoResult for Result<Value, Error> {
	fn into_result(self) -> Result<Value, Error> {
		self
	}
}

impl IntoResult for bool {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::Bool(self))
	}
}

impl IntoResult for Result<bool, Error> {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::Bool(self?))
	}
}

impl IntoResult for f64 {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::Number(self))
	}
}

impl IntoResult for Result<f64, Error> {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::Number(self?))
	}
}

impl IntoResult for String {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::String(self))
	}
}

impl IntoResult for Result<String, Error> {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::String(self?))
	}
}
