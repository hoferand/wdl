use std::{collections::HashMap, fmt::Debug, sync::Arc};

use ast::Function;
use futures::future::BoxFuture;

use crate::{wdl_std::StdFunction, Error};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Null,
	Bool(bool),
	Number(f64),
	String(String),
	Array(Vec<Value>),
	Object(HashMap<String, Value>),
	Function(FunctionValue),
}

#[derive(Clone)]
pub(crate) enum FunctionValue {
	Custom(Function),
	Std(Arc<dyn Fn(Value) -> BoxFuture<'static, Result<Value, Error>> + Send + Sync>),
	Magic(Arc<dyn StdFunction + Send + Sync>),
}

impl PartialEq for FunctionValue {
	fn eq(&self, _: &Self) -> bool {
		false
	}
}

impl Debug for FunctionValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Function")
	}
}

impl Value {
	pub fn boolify(&self) -> bool {
		match self {
			Value::Null => false,
			Value::Bool(b) => *b,
			Value::Number(n) => *n != 0.0,
			Value::String(s) => !s.is_empty(),
			Value::Array(a) => !a.is_empty(),
			Value::Object(o) => !o.is_empty(),
			Value::Function(_) => true,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Value::Null => "null",
			Value::Bool(_) => "bool",
			Value::Number(_) => "number",
			Value::String(_) => "string",
			Value::Array(_) => "array",
			Value::Object(_) => "object",
			Value::Function(_) => "function",
		}
		.to_owned()
	}
}

impl ToString for Value {
	fn to_string(&self) -> String {
		match self {
			Value::Null => "null".to_owned(),
			Value::Bool(b) => b.to_string(),
			Value::Number(n) => n.to_string(),
			Value::String(s) => s.to_owned(),
			Value::Array(a) => {
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
			Value::Object(o) => {
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
			Value::Function(_) => "function".to_owned(),
		}
	}
}

impl PartialOrd for Value {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		// TODO: rethink
		match (self, other) {
			(Value::Number(n1), Value::Number(n2)) => n1.partial_cmp(n2),
			_ => None,
		}
	}
}
