use ast::{ScopedIdentifier, Span};

use crate::{wdl_std::get_handler, Channel, Error, Value};

pub fn resolve_id(id: &ScopedIdentifier<Span>) -> Option<Value> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.val.0.as_str() {
		"new" => Some(get_handler(new)),
		"close" => Some(get_handler(close)),
		_ => None,
	}
}

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
