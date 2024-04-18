use crate::{
	wdl_std::{get_handler, Arg},
	FunctionId, FunctionValue, Value,
};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.0.as_str() {
		"print" => Some(get_handler(print)),
		_ => None,
	}
}

pub async fn print(arg: Arg<Value>) {
	println!("{}", arg.val.to_string());
}
