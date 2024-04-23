use crate::{
	wdl_std::{get_handler, Source},
	Error, ErrorKind, FunctionId, FunctionValue,
};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.0.as_str() {
		"done" => Some(get_handler(done)),
		"cancel" => Some(get_handler(cancel)),
		_ => None,
	}
}

async fn done(Source(src): Source) -> Result<(), Error> {
	Err(Error {
		kind: ErrorKind::OrderDone,
		src: Some(src),
	})
}

async fn cancel(Source(src): Source) -> Result<(), Error> {
	Err(Error {
		kind: ErrorKind::OrderCancel,
		src: Some(src),
	})
}
