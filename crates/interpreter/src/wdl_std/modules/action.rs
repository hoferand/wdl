use ast::ScopedIdentifier;

use crate::{wdl_std::get_handler, Value};

pub fn resolve_id(id: &ScopedIdentifier) -> Option<Value> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.val.0.as_str() {
		"pickup" => Some(get_handler(pickup)),
		"drop" => Some(get_handler(drop)),
		"drive" => Some(get_handler(drive)),
		_ => None,
	}
}

async fn pickup() {
	todo!()
}

async fn drop() {
	todo!()
}

async fn drive() {
	todo!()
}
