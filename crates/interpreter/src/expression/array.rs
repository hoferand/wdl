use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Array, Node};

use crate::{Environment, Error, Scope, Value};

use super::interpret_expression;

#[async_recursion]
pub async fn interpret_array(
	expr: &Node<Array>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	let mut values = Vec::new();

	for val_expr in &expr.val.values {
		values.push(interpret_expression(val_expr, scope, env).await?);
	}

	Ok(Value::Array(values))
}
