pub mod type_;
pub use type_::*;
pub mod channel_id;
pub use channel_id::ChannelId;
pub mod function_id;
pub use function_id::FunctionId;

use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
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
			Self::String(s) => s.to_owned(),
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

					out.push_str(id); // TODO: fix ids like "my key"
					out.push_str(": ");
					out.push_str(&val.to_string());
				}
				out.push('}');
				out
			}
			Self::Function(fn_id) => format!("<function `{}`>", fn_id),
			Self::Channel(ch_id) => format!("<channel `{}`>", ch_id.id),
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
