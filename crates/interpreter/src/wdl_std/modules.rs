mod action;
mod channel;
mod debug;
mod http;
mod order;
mod regex;
mod test;
mod time;

use crate::{FunctionId, FunctionValue};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if let Some(module) = id.scope.first() {
		return match module.0.as_str() {
			"action" => action::resolve_id(id),
			"debug" => debug::resolve_id(id),
			"http" => http::resolve_id(id),
			"time" => time::resolve_id(id),
			"regex" => regex::resolve_id(id),
			"channel" => channel::resolve_id(id),
			"order" => order::resolve_id(id),
			"test" => test::resolve_id(id),
			_ => None,
		};
	}

	// TODO: add prelude which do not need a scope

	None
}
