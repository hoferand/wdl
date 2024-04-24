use crate::{
	wdl_std::{get_handler, id, Arg, Source},
	FunctionId, FunctionValue, Value,
};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.0.as_str() {
		"info" => Some(get_handler(info)),
		"warning" => Some(get_handler(warning)),
		"error" => Some(get_handler(error)),
		_ => None,
	}
}

pub async fn info(Source(src): Source, msg: Arg<Value, { id(b"msg") }>) {
	println!(
		"INFO[{}:{}]: {}",
		src.start.line + 1,
		src.start.column,
		truncate(msg.val.to_string(), 100),
	);
}

pub async fn warning(Source(src): Source, msg: Arg<Value, { id(b"msg") }>) {
	println!(
		"WARNING[{}:{}]: {}",
		src.start.line + 1,
		src.start.column,
		truncate(msg.val.to_string(), 100),
	);
}

pub async fn error(Source(src): Source, msg: Arg<Value, { id(b"msg") }>) {
	println!(
		"ERROR[{}:{}]: {}",
		src.start.line + 1,
		src.start.column,
		truncate(msg.val.to_string(), 100),
	);
}

/// `len` must be >= 3
fn truncate(s: String, len: usize) -> String {
	if s.chars().count() <= len {
		return s;
	}

	s.chars().take(len - 3).collect::<String>() + "..."
}
