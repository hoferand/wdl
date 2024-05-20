use std::fmt::Display;

use serde::Serialize;

use ast::Span;

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
	pub msg: String,
	pub level: LogEntryLevel,
	pub user: bool,
	pub span: Option<Span>,
}

impl LogEntry {
	pub fn trace(msg: impl Into<String>, span: Option<Span>) -> Self {
		Self {
			msg: msg.into(),
			level: LogEntryLevel::Trace,
			user: false,
			span,
		}
	}

	pub fn debug(msg: impl Into<String>, span: Option<Span>) -> Self {
		Self {
			msg: msg.into(),
			level: LogEntryLevel::Debug,
			user: false,
			span,
		}
	}

	pub fn info(msg: impl Into<String>, span: Option<Span>) -> Self {
		Self {
			msg: msg.into(),
			level: LogEntryLevel::Info,
			user: false,
			span,
		}
	}

	pub fn warn(msg: impl Into<String>, span: Option<Span>) -> Self {
		Self {
			msg: msg.into(),
			level: LogEntryLevel::Warn,
			user: false,
			span,
		}
	}

	pub fn error(msg: impl Into<String>, span: Option<Span>) -> Self {
		Self {
			msg: msg.into(),
			level: LogEntryLevel::Error,
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
