use ast::Function;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Null,
	Bool(bool),
	Number(f64),
	String(String),
	Function(FunctionValue),
}

#[derive(Debug, Clone)]
pub enum FunctionValue {
	Custom(Function),
	_Std,
}

impl PartialEq for FunctionValue {
	fn eq(&self, _: &Self) -> bool {
		false
	}
}

impl Value {
	pub fn boolify(&self) -> bool {
		match self {
			Value::Null => false,
			Value::Bool(b) => *b,
			Value::Number(n) => *n != 0.0,
			Value::String(s) => !s.is_empty(),
			Value::Function(_) => true,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Value::Null => "null",
			Value::Bool(_) => "bool",
			Value::Number(_) => "number",
			Value::String(_) => "string",
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
