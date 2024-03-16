use std::collections::HashMap;

use crate::Value;

pub(crate) trait IntoValue {
	fn into_value(self) -> Value;
}

impl IntoValue for Value {
	fn into_value(self) -> Value {
		self
	}
}

impl IntoValue for () {
	fn into_value(self) -> Value {
		Value::Null
	}
}

impl IntoValue for bool {
	fn into_value(self) -> Value {
		Value::Bool(self)
	}
}

impl IntoValue for f64 {
	fn into_value(self) -> Value {
		Value::Number(self)
	}
}

impl IntoValue for String {
	fn into_value(self) -> Value {
		Value::String(self)
	}
}

impl<T: IntoValue> IntoValue for Vec<T> {
	fn into_value(self) -> Value {
		Value::Array(self.into_iter().map(IntoValue::into_value).collect())
	}
}

impl<T: IntoValue> IntoValue for HashMap<String, T> {
	fn into_value(self) -> Value {
		Value::Object(self.into_iter().map(|(k, v)| (k, v.into_value())).collect())
	}
}

impl<T: IntoValue> IntoValue for Option<T> {
	fn into_value(self) -> Value {
		self.map(IntoValue::into_value).unwrap_or(Value::Null)
	}
}
