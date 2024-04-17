use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Offset, Span};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_offset(
	expr: &Node<Span, Offset<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Value, Error> {
	let value = interpret_expr(&expr.val.value, env, g_env).await?;

	let offset = interpret_expr(&expr.val.offset, env, g_env).await?;

	let val = match (&value, &offset) {
		(Value::Array(a), Value::Number(n)) => a.get(*n as usize).unwrap_or(&Value::Null).clone(), // TODO: fix `*n as usize`
		// TODO: (Value::String(s), Value::Number(n)) => todo!(),
		(Value::Object(o), Value::String(s)) => o.get(s).unwrap_or(&Value::Null).clone(),
		_ => {
			return Err(Error::InvalidType {
				msg: format!("`{}`[`{}`]", value.get_type(), offset.get_type()),
				span: expr.src.clone(),
			});
		}
	};

	Ok(val)
}
