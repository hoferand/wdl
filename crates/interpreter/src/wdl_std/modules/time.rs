use std::time::Duration;

use ast::ScopedIdentifier;

use crate::{wdl_std::get_handler, Value};

pub fn resolve_id(id: &ScopedIdentifier) -> Option<Value> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.val.0.as_str() {
		"sleep" => Some(get_handler(sleep)),
		_ => None,
	}
}

pub async fn sleep(millis: f64) {
	tokio::time::sleep(Duration::from_millis(millis as u64)).await; // TODO: fix millis as u64
}
