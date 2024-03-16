use crate::{Error, Value};

use super::IntoValue;

pub(crate) trait IntoResult {
	fn into_result(self) -> Result<Value, Error>;
}

impl<T: IntoValue> IntoResult for T {
	fn into_result(self) -> Result<Value, Error> {
		Ok(self.into_value())
	}
}

impl<T: IntoValue> IntoResult for Result<T, Error> {
	fn into_result(self) -> Result<Value, Error> {
		Ok(self?.into_value())
	}
}
