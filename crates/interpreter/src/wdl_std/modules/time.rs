use std::time::Duration;

use crate::{
	wdl_std::{get_handler, id, Arg},
	FunctionId, FunctionValue,
};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.0.as_str() {
		"sleep" => Some(get_handler(sleep)),
		_ => None,
	}
}

pub async fn sleep(millis: Arg<f64, { id(b"millis") }>) {
	tokio::time::sleep(Duration::from_millis(millis.val as u64)).await; // TODO: fix cast
}
