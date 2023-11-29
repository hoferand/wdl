use crate::{Error, Value};

pub(crate) trait IntoResult {
	fn into_result(self) -> Result<Value, Error>;
}

impl IntoResult for () {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::Null)
	}
}

impl IntoResult for Value {
	fn into_result(self) -> Result<Value, Error> {
		Ok(self)
	}
}

impl IntoResult for Result<Value, Error> {
	fn into_result(self) -> Result<Value, Error> {
		self
	}
}
