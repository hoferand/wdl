use crate::{
	wdl_std::{get_handler, Arg, Env},
	ChannelId, Error, FunctionId, FunctionValue,
};

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

pub async fn new(Env(env): Env, buffer: Arg<f64>) -> Result<ChannelId, Error> {
	if buffer.val < 1.0 {
		// TODO: improve error message
		return Err(Error::fatal(format!(
			"The buffer size for a channel must be at least `1`, but `{}` given",
			buffer.val
		)));
	}

	let (ch_id, _) = env.create_ch(buffer.val as usize).await; // TODO: fix cast

	Ok(ch_id)
}

pub async fn close(Env(env): Env, ch_id: Arg<ChannelId>) -> Result<(), Error> {
	let Some(ch) = env.get_ch(&ch_id.val).await else {
		return Err(Error::fatal(format!("Channel `{}` not found", ch_id.val.0)));
	};

	ch.close().await;

	Ok(())
}
