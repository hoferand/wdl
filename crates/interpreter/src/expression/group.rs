use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Group, Node};

use crate::{Environment, Error, Scope, Value};

use super::interpret_expression;

#[async_recursion]
pub async fn interpret_group(
	expr: &Node<Group>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	interpret_expression(&expr.val.expression, scope, env).await
}
