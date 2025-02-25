use std::time::Duration;

use crate::{
	FunctionId, FunctionValue,
	wdl_std::{Arg, get_handler, id},
};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.id.as_str() {
		"sleep" => Some(get_handler(sleep)),
		_ => None,
	}
}

pub async fn sleep(ms: Arg<f64, { id(b"ms") }>) {
	tokio::time::sleep(Duration::from_millis(ms.val as u64)).await; // TODO: fix cast
}
