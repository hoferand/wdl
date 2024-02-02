use async_recursion::async_recursion;

use ast::{Array, Node};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_array(
	expr: &Node<Array>,
	env: &Environment,
	g_env: &Environment,
) -> Result<Value, Error> {
	let mut values = Vec::new();

	for val_expr in &expr.val.values {
		values.push(interpret_expr(val_expr, env, g_env).await?);
	}

	Ok(Value::Array(values))
}
