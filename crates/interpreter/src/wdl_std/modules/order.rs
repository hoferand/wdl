use ast::Span;

use crate::{wdl_std::get_handler, Error, ErrorKind, FunctionId, FunctionValue};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.id.as_str() {
		"done" => Some(get_handler(done)),
		"cancel" => Some(get_handler(cancel)),
		_ => None,
	}
}

async fn done(fn_span: Span) -> Result<(), Error> {
	Err(Error {
		kind: ErrorKind::OrderDone,
		span: Some(fn_span),
	})
}

async fn cancel(fn_span: Span) -> Result<(), Error> {
	Err(Error {
		kind: ErrorKind::OrderCancel,
		span: Some(fn_span),
	})
}
