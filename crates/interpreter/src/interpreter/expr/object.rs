use std::collections::HashMap;

use async_recursion::async_recursion;

use ast::{Node, Object};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_object(
	expr: &Node<Object>,
	env: &Environment,
	g_env: &Environment,
) -> Result<Value, Error> {
	let mut values = HashMap::new();

	for (key, val_expr) in &expr.val.values {
		values.insert(key.to_owned(), interpret_expr(val_expr, env, g_env).await?);
	}

	Ok(Value::Object(values))
}
