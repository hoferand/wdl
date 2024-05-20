use std::sync::Arc;

use ast::Span;

use crate::{
	wdl_std::{get_handler, id, Arg},
	Environment, FunctionId, FunctionValue, LogEntry, LogEntryLevel, Value,
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

pub async fn info(msg: Arg<Value, { id(b"msg") }>, fn_span: Span, env: Arc<Environment>) {
	env.send_log(LogEntry {
		msg: truncate(msg.val.to_string(), 100),
		level: LogEntryLevel::Info,
		user: true,
		span: Some(fn_span),
	})
	.await;
}

pub async fn warn(msg: Arg<Value, { id(b"msg") }>, fn_span: Span, env: Arc<Environment>) {
	env.send_log(LogEntry {
		msg: truncate(msg.val.to_string(), 100),
		level: LogEntryLevel::Warn,
		user: true,
		span: Some(fn_span),
	})
	.await;
}

pub async fn error(msg: Arg<Value, { id(b"msg") }>, fn_span: Span, env: Arc<Environment>) {
	env.send_log(LogEntry {
		msg: truncate(msg.val.to_string(), 100),
		level: LogEntryLevel::Error,
		user: true,
		span: Some(fn_span),
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
