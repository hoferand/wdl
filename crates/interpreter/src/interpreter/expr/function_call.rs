use async_recursion::async_recursion;

use ast::{FunctionCall, Node};

use crate::{
	interpreter::stmt, interrupt::Interrupt, value::FunctionValue, Environment, Error, Value,
};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_function_call(
	expr: &Node<FunctionCall>,
	env: &Environment,
	g_env: &Environment,
) -> Result<Value, Error> {
	let function_val = match interpret_expr(&expr.val.function, env, g_env).await? {
		Value::Function(f) => f,
		v => {
			return Err(Error::InvalidType {
				msg: format!("`{}`()", v.get_type()),
				span: expr.val.function.get_span().clone(),
			})
		}
	};

	let val;
	match function_val {
		FunctionValue::Custom(function) => {
			let inner_env = Environment::with_parent(g_env);

			let mut ids = function.parameter.iter();
			let mut vals = expr.val.parameter.val.iter();
			loop {
				match (ids.next(), vals.next()) {
					(None, Some(_)) | (Some(_), None) => {
						return Err(Error::ArityMismatch {
							expected: function.parameter.len(),
							given: expr.val.parameter.val.len(),
							span: expr.val.parameter.span.clone(),
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
				int @ Interrupt::Continue | int @ Interrupt::Break => {
					return Err(Error::Fatal(format!(
						"AST invalid, {} inside of function found",
						int.get_type()
					)));
				}
			}
		}
		FunctionValue::_Std => todo!(),
	}

	Ok(val)
}
