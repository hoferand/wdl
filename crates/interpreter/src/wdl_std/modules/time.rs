use std::{sync::Arc, time::Duration};

use crate::{wdl_std::get_handler, Environment, Error, FunctionId, FunctionValue};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.0.as_str() {
		"sleep" => Some(get_handler(sleep)),
		_ => None,
	}
}

pub async fn sleep(_env: Arc<Environment>, millis: f64) -> Result<(), Error> {
	tokio::time::sleep(Duration::from_millis(millis as u64)).await; // TODO: fix cast
	Ok(())
}
