use ast::ScopedIdentifier;

use crate::{wdl_std::get_handler, Value};

pub fn resolve_id(id: &ScopedIdentifier) -> Option<Value> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.val.0.as_str() {
		"print" => Some(get_handler(print)),
		_ => None,
	}
}

pub async fn print(val: Value) {
	println!("{}", val.to_string());
}
