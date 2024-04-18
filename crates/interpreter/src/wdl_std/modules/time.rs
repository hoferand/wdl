use std::time::Duration;

use crate::{wdl_std::get_handler, Error, FunctionId, FunctionValue};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.0.as_str() {
		"sleep" => Some(get_handler(sleep)),
		_ => None,
	}
}

pub async fn sleep(millis: u64) -> Result<(), Error> {
	tokio::time::sleep(Duration::from_millis(millis)).await;
	Ok(())
}
