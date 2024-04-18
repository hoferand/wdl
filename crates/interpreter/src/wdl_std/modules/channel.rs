use crate::{FunctionId, FunctionValue};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.0.as_str() {
		//"new" => Some(get_handler(new)),
		//"close" => Some(get_handler(close)),
		_ => None,
	}
}
/*
pub async fn new(buffer: f64) -> Result<Channel, Error> {
	if buffer < 1.0 {
		// TODO: improve error message
		return Err(Error::fatal(format!(
			"The buffer size for a channel must be at least `1`, but `{}` given",
			buffer
		)));
	}

	Ok(Channel::new(buffer as usize)) // TODO: fix cast
}

pub async fn close(ch: Channel) -> Result<(), Error> {
	ch.close().await;

	Ok(())
}
*/
