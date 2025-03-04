use std::{collections::HashMap, sync::Arc};

use ast::{Identifier, Span};

use crate::{
	Environment, Error, ErrorKind, FunctionId, FunctionValue, Value, expression::run_function,
};

mod call_context;
pub use call_context::*;
mod modules;
pub use modules::*;
mod std_function;
pub use std_function::*;

mod arg_type;
use arg_type::ArgType;
mod arg;
use arg::*;
mod from_call_context;
use from_call_context::FromCallContext;
mod from_value;
use from_value::FromValue;
mod handler_function;
use handler_function::HandlerFunction;
mod handler;
use handler::Handler;
mod into_result;
use into_result::IntoResult;
mod result_type;
use result_type::ResultType;

fn get_handler<H, T>(fun: H) -> FunctionValue
where
	H: Handler<T> + Clone + 'static + Sync,
	T: 'static,
{
	let hf = HandlerFunction {
		handler: fun,
		call: |h, ctx, strict| h.call(ctx, strict),
	};

	FunctionValue::Std(Arc::new(hf))
}

// TODO: make args and return type generic
async fn call_function(
	function_id: &FunctionId,
	values: Vec<Value>,
	callback_name: Identifier,
	span: Span,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	let mut args = Vec::new();

	for val in values {
		args.push(ArgumentValue { idx: 1, span, val });
	}

	let error = match run_function(function_id, span, args, HashMap::new(), false, env).await {
		Ok(val) => return Ok(val),
		Err(err) => err,
	};
	match error {
		Error {
			kind: ErrorKind::ArityMismatch { expected, given },
			span: src,
		} => Err(Error {
			kind: ErrorKind::Fatal(format!(
				"Callback for `{}` should require `{}` argument(s) but requires `{}` argument(s)",
				callback_name, given, expected
			)),
			span: src,
		}),
		Error {
			kind: ErrorKind::MissingArgument { id },
			span: src,
		} => Err(Error {
			kind: ErrorKind::Fatal(format!(
				"Callback for `{}` should not require argument `{}`",
				callback_name, id
			)),
			span: src,
		}),
		err => Err(err),
	}
}
