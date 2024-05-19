use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Offset};

use crate::{expression::interpret_expression, Environment, Error, ErrorKind, Scope, Value};

#[async_recursion]
pub async fn interpret_offset(
	expr: &Node<Offset>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	let value = interpret_expression(&expr.val.value, scope, env).await?;

	let offset = interpret_expression(&expr.val.offset, scope, env).await?;

	let val = match (&value, &offset) {
		(Value::Array(a), Value::Number(n)) => a.get(*n as usize).unwrap_or(&Value::Null).clone(), // TODO: fix cast
		(Value::String(s), Value::Number(n)) => s
			.chars()
			.nth(*n as usize)
			.map(|ch| Value::String(ch.to_string()))
			.unwrap_or(Value::Null), // TODO: fix cast
		(Value::Object(o), Value::String(s)) => o.get(s).unwrap_or(&Value::Null).clone(),
		_ => {
			return Err(Error {
				kind: ErrorKind::InvalidType {
					msg: format!("`{}`[`{}`]", value.get_type(), offset.get_type()),
				},
				span: Some(expr.span),
			});
		}
	};

	Ok(val)
}
