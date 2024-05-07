use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserLog {
	pub msg: String,
	pub level: UserLogLevel,
}

impl UserLog {
	pub fn info(msg: impl Into<String>) -> Self {
		Self {
			msg: msg.into(),
			level: UserLogLevel::Info,
		}
	}

	pub fn warn(msg: impl Into<String>) -> Self {
		Self {
			msg: msg.into(),
			level: UserLogLevel::Warn,
		}
	}

	pub fn error(msg: impl Into<String>) -> Self {
		Self {
			msg: msg.into(),
			level: UserLogLevel::Error,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum UserLogLevel {
	Info,
	Warn,
	Error,
}

impl Display for UserLogLevel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			UserLogLevel::Info => write!(f, "INFO"),
			UserLogLevel::Warn => write!(f, "WARN"),
			UserLogLevel::Error => write!(f, "ERROR"),
		}
	}
}
