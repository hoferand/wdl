use std::fmt::Display;

use serde::Serialize;

use ast::Span;

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
	pub level: LogEntryLevel,
	pub msg: String,
	pub user: bool,
	pub span: Option<Span>,
}

impl LogEntry {
	pub fn trace(msg: impl Into<String>, span: Option<Span>) -> Self {
		Self {
			level: LogEntryLevel::Trace,
			msg: msg.into(),
			user: false,
			span,
		}
	}

	pub fn debug(msg: impl Into<String>, span: Option<Span>) -> Self {
		Self {
			level: LogEntryLevel::Debug,
			msg: msg.into(),
			user: false,
			span,
		}
	}

	pub fn info(msg: impl Into<String>, span: Option<Span>) -> Self {
		Self {
			level: LogEntryLevel::Info,
			msg: msg.into(),
			user: false,
			span,
		}
	}

	pub fn warn(msg: impl Into<String>, span: Option<Span>) -> Self {
		Self {
			level: LogEntryLevel::Warn,
			msg: msg.into(),
			user: false,
			span,
		}
	}

	pub fn error(msg: impl Into<String>, span: Option<Span>) -> Self {
		Self {
			level: LogEntryLevel::Error,
			msg: msg.into(),
			user: false,
			span,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LogEntryLevel {
	Error,
	Warn,
	Info,
	Debug,
	Trace,
}

impl Display for LogEntryLevel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Error => write!(f, "ERROR"),
			Self::Warn => write!(f, "WARN"),
			Self::Info => write!(f, "INFO"),
			Self::Debug => write!(f, "DEBUG"),
			Self::Trace => write!(f, "TRACE"),
		}
	}
}
