mod sleep;
use std::sync::Arc;

use sleep::sleep;
mod print;
use print::print;

use crate::value::{FunctionValue, Value};

pub fn get_std(id: &str) -> Option<Value> {
	match id {
		"printfn" => Some(Value::Function(FunctionValue::Std(Arc::new(print)))),
		"sleepfn" => Some(Value::Function(FunctionValue::Std(Arc::new(sleep)))),
		_ => None,
	}
}
