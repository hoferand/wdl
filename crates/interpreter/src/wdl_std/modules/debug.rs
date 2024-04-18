use crate::{wdl_std::get_handler, Error, FunctionId, FunctionValue, Value};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.0.as_str() {
		"print" => Some(get_handler(print)),
		_ => None,
	}
}

pub async fn print(val: Value) -> Result<(), Error> {
	println!("{}", val.to_string());
	Ok(())
}
