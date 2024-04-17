use serde::Serialize;

use crate::{Channel, Error, Value};

pub(crate) trait IntoResult {
	fn into_result(self) -> Result<Value, Error>;
}

impl<T: Serialize> IntoResult for Result<T, Error> {
	fn into_result(self) -> Result<Value, Error> {
		let json_val = match serde_json::to_value(self?) {
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

impl IntoResult for Result<Channel, Error> {
	fn into_result(self) -> Result<Value, Error> {
		Ok(Value::Channel(self?))
	}
}
