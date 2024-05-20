use crate::{
	wdl_std::{get_handler, id, Arg, Env, Source},
	FunctionId, FunctionValue, LogEntry, LogEntryLevel, Value,
};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.id.as_str() {
		"info" => Some(get_handler(info)),
		"warn" => Some(get_handler(warn)),
		"error" => Some(get_handler(error)),
		_ => None,
	}
}

pub async fn info(Source(src): Source, msg: Arg<Value, { id(b"msg") }>, Env(env): Env) {
	env.send_log(LogEntry {
		msg: truncate(msg.val.to_string(), 100),
		level: LogEntryLevel::Info,
		user: true,
		span: Some(src),
	})
	.await;
}

pub async fn warn(Source(src): Source, msg: Arg<Value, { id(b"msg") }>, Env(env): Env) {
	env.send_log(LogEntry {
		msg: truncate(msg.val.to_string(), 100),
		level: LogEntryLevel::Warn,
		user: true,
		span: Some(src),
	})
	.await;
}

pub async fn error(Source(src): Source, msg: Arg<Value, { id(b"msg") }>, Env(env): Env) {
	env.send_log(LogEntry {
		msg: truncate(msg.val.to_string(), 100),
		level: LogEntryLevel::Error,
		user: true,
		span: Some(src),
	})
	.await;
}

/// `len` must be >= 3
fn truncate(s: String, len: usize) -> String {
	if s.chars().count() <= len {
		return s;
	}

	s.chars().take(len - 3).collect::<String>() + "..."
}
