mod action;
mod debug;
mod http;
mod regex;
mod time;

use ast::ScopedIdentifier;

use crate::Value;

pub fn resolve_id(id: &ScopedIdentifier) -> Option<Value> {
	if let Some(module) = id.scope.first() {
		return match module.val.0.as_str() {
			"action" => action::resolve_id(id),
			"debug" => debug::resolve_id(id),
			"http" => http::resolve_id(id),
			"time" => time::resolve_id(id),
			"regex" => regex::resolve_id(id),
			_ => None,
		};
	}

	// TODO: add prelude which do not need a scope

	None
}
