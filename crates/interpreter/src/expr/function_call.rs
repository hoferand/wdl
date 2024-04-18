use std::sync::Arc;

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

	let val;
	match function_val {
		FunctionValue::Custom(function) => {
			let inner_env = Arc::new(Environment::with_parent(Arc::clone(g_env)));

			let mut ids = function.parameter.iter();
			let mut vals = expr.val.parameter.val.iter();
			loop {
				match (ids.next(), vals.next()) {
					(None, Some(_)) | (Some(_), None) => {
						return Err(Error {
							kind: ErrorKind::ArityMismatch {
								expected: function.parameter.len(),
								given: expr.val.parameter.val.len(),
							},
							src: Some(expr.val.parameter.src.clone()),
						});
					}
					(Some(id_node), Some(val_expr)) => {
						let id = id_node.val.id.clone();
						let val = interpret_expr(val_expr, env, g_env).await?;
						inner_env.declare(id, val).await?;
					}
					_ => break,
				}
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
			let mut args = Vec::new();
			for (idx, arg) in expr.val.parameter.val.iter().enumerate() {
				args.push(ArgumentValue {
					idx: idx + 1,
					span: arg.get_src().clone(),
					val: interpret_expr(arg, env, g_env).await?,
				});
			}

			let args = args.into_iter();
			val = std_fn
				.call_with_ctx(CallContext {
					fn_span: expr.val.function.get_src().clone(),
					env: Arc::clone(g_env),
					args,
				})
				.await?;
		}
	}

	Ok(val)
}
