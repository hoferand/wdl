use async_recursion::async_recursion;

use ast::{Assignment, Node};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_assignment(
	expr: &Node<Assignment>,
	env: &Environment,
	g_env: &Environment,
) -> Result<Value, Error> {
	let value = interpret_expr(&expr.val.value, env, g_env).await?;
	let id = expr.val.id.clone();
	env.assign(id, value.clone()).await?;

	Ok(value)
}
