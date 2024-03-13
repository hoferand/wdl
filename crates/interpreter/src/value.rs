use std::{collections::HashMap, fmt::Debug, sync::Arc};

use ast::Function;

use crate::wdl_std::StdFunction;

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
pub enum FunctionValue {
	Custom(Function),
	Std(Arc<dyn StdFunction + Send + Sync>),
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
			Self::Null => false,
			Self::Bool(b) => *b,
			Self::Number(n) => *n != 0.0,
			Self::String(s) => !s.is_empty(),
			Self::Array(a) => !a.is_empty(),
			Self::Object(o) => !o.is_empty(),
			Self::Function(_) => true,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Self::Null => "null",
			Self::Bool(_) => "bool",
			Self::Number(_) => "number",
			Self::String(_) => "string",
			Self::Array(_) => "array",
			Self::Object(_) => "object",
			Self::Function(_) => "function",
		}
		.to_owned()
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

					out.push_str(id);
					out.push_str(": ");
					out.push_str(&val.to_string());
				}
				out.push('}');
				out
			}
			Self::Function(_) => "function".to_owned(),
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
