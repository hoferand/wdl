mod from_call_context;
use from_call_context::FromCallContext;
mod handler_function;
use handler_function::HandlerFunction;
mod handler;
use handler::Handler;
mod into_result;
use into_result::IntoResult;
mod arg_type;
mod result_type;

pub(crate) mod arguments;
pub(crate) use arguments::*;
pub(crate) mod modules;
pub(crate) use modules::resolve_id;
pub(crate) mod std_function;
pub(crate) use std_function::StdFunction;

use std::sync::Arc;

use crate::FunctionValue;

fn get_handler<H, T>(fun: H) -> FunctionValue
where
	H: Handler<T> + Clone + 'static + Sync,
	T: 'static,
{
	let hf = HandlerFunction {
		handler: fun,
		call: |h, ctx| h.call(ctx),
	};

	FunctionValue::Std(Arc::new(hf))
}
