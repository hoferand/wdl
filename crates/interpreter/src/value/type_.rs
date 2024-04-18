use std::fmt::Display;

pub enum ValueType {
	Null,
	Bool,
	Number,
	String,
	Array,
	Object,
	Function,
	Channel,
}

impl Display for ValueType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ValueType::Null => write!(f, "null"),
			ValueType::Bool => write!(f, "bool"),
			ValueType::Number => write!(f, "number"),
			ValueType::String => write!(f, "string"),
			ValueType::Array => write!(f, "array"),
			ValueType::Object => write!(f, "object"),
			ValueType::Function => write!(f, "function"),
			ValueType::Channel => write!(f, "channel"),
		}
	}
}
