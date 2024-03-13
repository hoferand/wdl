use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Group, Node};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_group(
	expr: &Node<Group>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Value, Error> {
	interpret_expr(&expr.val.expression, env, g_env).await
}
