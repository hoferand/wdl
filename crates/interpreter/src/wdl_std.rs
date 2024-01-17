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
	Some(Value::Function(FunctionValue::Std(match id {
		"print" => get_handler(handlers::print),
		"sleep" => get_handler(handlers::sleep),
		"test" => get_handler(test),
		"get_string" => get_handler(get_string),
		"get_number" => get_handler(get_number),
		"optional" => get_handler(optional),
		_ => return None,
	})))
}

async fn optional(arg: Option<f64>) {
	if let Some(arg) = arg {
		println!("{}", arg);
	} else {
		println!("No argument given!");
	}
}

async fn test() -> f64 {
	println!("test");
	23.0
}

async fn get_string() -> String {
	"My String".to_owned()
}

async fn get_number() -> f64 {
	34.0
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
