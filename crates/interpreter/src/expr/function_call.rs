use std::{collections::HashMap, sync::Arc};

use async_recursion::async_recursion;

use ast::{FunctionCall, Node, Span};

use crate::{
	stmt,
	wdl_std::{ArgumentValue, CallContext},
	Environment, Error, ErrorKind, FunctionValue, Interrupt, Value,
};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_function_call(
	expr: &Node<Span, FunctionCall<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Value, Error> {
	let function_id = match interpret_expr(&expr.val.function, env, g_env).await? {
		Value::Function(f) => f,
		v => {
			return Err(Error {
				kind: ErrorKind::InvalidType {
					msg: format!("`{}`()", v.get_type()),
				},
				src: Some(expr.val.function.get_src().clone()),
			});
		}
	};

	let Some(function_val) = g_env.get_fn(&function_id).await else {
		return Err(Error::fatal(format!(
			"Function `{}` not found",
			function_id
		)));
	};

	let mut args = Vec::new();
	let mut named_args = HashMap::new();
	for (idx, arg) in expr.val.args.iter().enumerate() {
		let val = interpret_expr(&arg.val.val, env, g_env).await?;

		if let Some(id) = &arg.val.id {
			named_args.insert(
				id.val.clone(),
				ArgumentValue {
					idx: idx + 1,
					span: arg.src.clone(),
					val,
				},
			);
		} else {
			args.push(ArgumentValue {
				idx: idx + 1,
				span: arg.val.val.get_src().clone(),
				val,
			});
		}
	}

	let val;
	match function_val {
		FunctionValue::Custom(function) => {
			let inner_env = Arc::new(Environment::with_parent(Arc::clone(g_env)));

			let mut vals = args.into_iter();
			for id in function.parameter.iter() {
				if let Some(val) = vals.next() {
					// positional argument
					inner_env.declare(id.val.id.clone(), val.val).await?;
				} else if let Some(val) = named_args.get(&id.val.id.val).cloned() {
					// named argument
					named_args.remove(&id.val.id.val);
					inner_env.declare(id.val.id.clone(), val.val).await?;
				} else {
					// parameter missing
					return Err(Error {
						kind: ErrorKind::MissingArgument {
							id: id.val.id.val.clone(),
						},
						src: Some(expr.val.function.get_src().clone()),
					});
				}
			}

			let rem = vals.count();
			let rem_named = named_args.keys().len();

			if let Some((id, arg)) = named_args.into_iter().next() {
				return Err(Error {
					kind: ErrorKind::UnknownArgument { id },
					src: Some(arg.span),
				});
			}

			if rem != 0 {
				let expected = function.parameter.len();
				return Err(Error {
					kind: ErrorKind::ArityMismatch {
						expected,
						given: expected + rem + rem_named,
					},
					src: Some(expr.val.function.get_src().clone()),
				});
			}

			match stmt::interpret_block(&function.body, &inner_env, g_env).await? {
				Interrupt::None => val = Value::Null,
				Interrupt::Return(ret_val) => val = ret_val,
				int @ (Interrupt::Continue | Interrupt::Break) => {
					return Err(Error::fatal(format!(
						"AST invalid, {} inside of function found",
						int.get_type()
					)));
				}
			}
		}
		FunctionValue::Std(std_fn) => {
			let args = args.into_iter();
			val = std_fn
				.call_with_ctx(CallContext {
					fn_span: expr.val.function.get_src().clone(),
					env: Arc::clone(g_env),
					args,
					named_args,
				})
				.await?;
		}
	}

	Ok(val)
}
