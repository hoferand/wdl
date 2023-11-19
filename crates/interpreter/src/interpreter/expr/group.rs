use async_recursion::async_recursion;

use ast::{Group, Node};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_group(
	expr: &Node<Group>,
	env: &Environment,
	g_env: &Environment,
) -> Result<Value, Error> {
	interpret_expr(&expr.val.expression, env, g_env).await
}
