mod from_arguments;
use from_arguments::FromArguments;
mod handler_function;
use handler_function::HandlerFunction;
mod handler;
use handler::Handler;
mod into_result;
use into_result::IntoResult;
mod handlers;
pub(crate) mod std_function;
pub(crate) use std_function::StdFunction;

use std::sync::Arc;

use crate::value::{FunctionValue, Value};

pub fn get_function(id: &str) -> Option<Value> {
	if id == "sleep" {
		return Some(Value::Function(FunctionValue::Std(Arc::new(
			handlers::sleep,
		))));
	}

	Some(Value::Function(FunctionValue::Magic(match id {
		"print" => get_handler(handlers::print),
		"test" => get_handler(test),
		_ => return None,
	})))
}

fn test() {
	println!("test");
}

pub(crate) fn get_handler<H, T>(fun: H) -> Arc<HandlerFunction<H>>
where
	H: Handler<T> + Clone + 'static,
	T: 'static,
{
	let hf = HandlerFunction {
		handler: fun,
		call: |h, ctx| h.call(ctx),
	};

	Arc::new(hf)
}
