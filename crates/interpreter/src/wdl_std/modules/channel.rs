use ast::ScopedIdentifier;

use crate::{channel::Channel, wdl_std::get_handler, Error, Value};

pub fn resolve_id(id: &ScopedIdentifier) -> Option<Value> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.val.0.as_str() {
		"new" => Some(get_handler(new)),
		_ => None,
	}
}

pub async fn new(buffer: usize) -> Result<Value, Error> {
	Ok(Value::Channel(Channel::new(buffer)))
}
