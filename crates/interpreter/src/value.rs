pub mod type_;
pub use type_::*;
pub mod channel_id;
pub use channel_id::ChannelId;
pub mod function_id;
pub use function_id::FunctionId;

use std::{collections::HashMap, fmt::Debug};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum Value {
	Null,
	Bool(bool),
	Number(f64),
	String(String),
	Array(Vec<Value>),
	Object(HashMap<String, Value>),
	Function(FunctionId),
	Channel(ChannelId),
}

impl Value {
	pub fn boolify(&self) -> bool {
		match self {
			Self::Null => false,
			Self::Bool(b) => *b,
			Self::Number(n) => *n != 0.0,
			Self::String(s) => !s.is_empty(),
			Self::Array(a) => !a.is_empty(),
			Self::Object(o) => !o.is_empty(),
			Self::Function(_) => true,
			Self::Channel(_) => true,
		}
	}

	pub fn get_type(&self) -> ValueType {
		match self {
			Self::Null => ValueType::Null,
			Self::Bool(_) => ValueType::Bool,
			Self::Number(_) => ValueType::Number,
			Self::String(_) => ValueType::String,
			Self::Array(_) => ValueType::String,
			Self::Object(_) => ValueType::Object,
			Self::Function(_) => ValueType::Function,
			Self::Channel(_) => ValueType::Channel,
		}
	}
}

impl ToString for Value {
	fn to_string(&self) -> String {
		match self {
			Self::Null => "null".to_owned(),
			Self::Bool(b) => b.to_string(),
			Self::Number(n) => n.to_string(),
			Self::String(s) => format!("\"{}\"", s.to_owned()),
			Self::Array(a) => {
				let mut out = String::new();
				out.push('[');
				let mut first = true;
				for val in a {
					if !first {
						out.push_str(", ");
					}
					first = false;

					out.push_str(&val.to_string());
				}
				out.push(']');
				out
			}
			Self::Object(o) => {
				let mut out = String::new();
				out.push('{');
				let mut first = true;
				for (id, val) in o {
					if !first {
						out.push_str(", ");
					}
					first = false;

					out.push_str(id);
					out.push_str(": ");
					out.push_str(&val.to_string());
				}
				out.push('}');
				out
			}
			Self::Function(_) => "function".to_owned(),
			Self::Channel(_) => "channel".to_owned(),
		}
	}
}

impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		// TODO: rethink
		match (self, other) {
			(Value::Null, Value::Null) => true,
			(Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
			(Value::Number(n1), Value::Number(n2)) => n1 == n2,
			(Value::String(s1), Value::String(s2)) => s1 == s2,
			(Value::Array(a1), Value::Array(a2)) => a1 == a2,
			(Value::Object(o1), Value::Object(o2)) => o1 == o2,
			_ => false,
		}
	}
}

impl PartialOrd for Value {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		// TODO: rethink
		match (self, other) {
			(Self::Number(n1), Self::Number(n2)) => n1.partial_cmp(n2),
			_ => None,
		}
	}
}

impl Serialize for Value {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			Self::Null => serializer.serialize_none(),
			Self::Bool(bool) => serializer.serialize_bool(*bool),
			Self::Number(nr) => serializer.serialize_f64(*nr),
			Self::String(str) => serializer.serialize_str(str),
			Self::Array(arr) => serializer.collect_seq(arr),
			Self::Object(obj) => serializer.collect_map(obj),
			Self::Function(_) => todo!(),
			Self::Channel(_) => todo!(),
		}
	}
}

impl<'de> Deserialize<'de> for Value {
	fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
	where
		D: Deserializer<'de>,
	{
		use serde_json::Value as V;
		serde_to_value(V::deserialize(deserializer)?)
	}
}

fn serde_to_value<E: de::Error>(val: serde_json::Value) -> Result<Value, E> {
	use serde_json::Value as V;
	Ok(match val {
		V::Null => Value::Null,
		V::Bool(b) => Value::Bool(b),
		V::Number(nr) => {
			if let Some(n) = nr.as_f64() {
				Value::Number(n)
			} else {
				return Err(de::Error::custom(format!(
					"Number `{}` to big for saving",
					nr
				)));
			}
		}
		V::String(s) => Value::String(s),
		V::Array(arr) => {
			let mut vec = Vec::new();

			for val in arr {
				vec.push(serde_to_value(val)?);
			}

			Value::Array(vec)
		}
		V::Object(obj) => {
			let mut map = HashMap::new();

			for (key, val) in obj {
				map.insert(key, serde_to_value(val)?);
			}

			Value::Object(map)
		}
	})
}
