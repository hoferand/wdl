use serde::Deserialize;

use logger::log;
use logger::Colorize;

use crate::{
	wdl_std::{get_handler, id, Arg, ArgType},
	FunctionId, FunctionValue,
};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.0.as_str() {
		"test" => Some(get_handler(test)),
		_ => None,
	}
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MyStruct {
	key: Either,
	fn_: FunctionId,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Either {
	S(String),
	N(f64),
}

impl<'de> ArgType<'de> for MyStruct {}

async fn test(arg1: Option<Arg<bool, { id(b"arg") }>>, arg: Arg<MyStruct, { id(b"arg") }>) {
	log!("{:?}", arg.val);
}
