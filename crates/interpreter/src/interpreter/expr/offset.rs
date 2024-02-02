use async_recursion::async_recursion;

use ast::{Node, Offset};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_offset(
	expr: &Node<Offset>,
	env: &Environment,
	g_env: &Environment,
) -> Result<Value, Error> {
	let value = interpret_expr(&expr.val.value, env, g_env).await?;

	let offset = interpret_expr(&expr.val.offset, env, g_env).await?;

	let val = match (&value, &offset) {
		(Value::Array(a), Value::Number(n)) => a.get(*n as usize).unwrap_or(&Value::Null).clone(), // TODO: fix `*n as usize`
		// TODO: (Value::String(s), Value::Number(n)) => todo!(),
		// TODO: (Value::Object(o), Value::String(s)) => todo!(),
		_ => {
			return Err(Error::InvalidType {
				msg: format!("{}[{}]", value.get_type(), offset.get_type()),
				span: expr.span.clone(),
			});
		}
	};

	Ok(val)
}
