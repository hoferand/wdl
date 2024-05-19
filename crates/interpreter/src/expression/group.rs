use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Group, Node};

use crate::{expression::interpret_expression, Environment, Error, Scope, Value};

#[async_recursion]
pub async fn interpret_group(
	expr: &Node<Group>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	interpret_expression(&expr.val.expression, scope, env).await
}
