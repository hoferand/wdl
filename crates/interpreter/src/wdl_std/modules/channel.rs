use std::sync::Arc;

use crate::{wdl_std::get_handler, ChannelId, Environment, Error, FunctionId, FunctionValue};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.0.as_str() {
		"new" => Some(get_handler(new)),
		"close" => Some(get_handler(close)),
		_ => None,
	}
}

pub async fn new(env: Arc<Environment>, buffer: f64) -> Result<ChannelId, Error> {
	if buffer < 1.0 {
		// TODO: improve error message
		return Err(Error::fatal(format!(
			"The buffer size for a channel must be at least `1`, but `{}` given",
			buffer
		)));
	}

	let (ch_id, _) = env.create_ch(buffer as usize).await; // TODO: fix cast

	Ok(ch_id)
}

pub async fn close(env: Arc<Environment>, ch_id: ChannelId) -> Result<(), Error> {
	let Some(ch) = env.get_ch(&ch_id).await else {
		return Err(Error::fatal(format!("Channel `{}` not found", ch_id.0)));
	};

	ch.close().await;

	Ok(())
}
