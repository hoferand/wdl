use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Group, Node};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_group(
	expr: &Node<Group>,
	env: &RwLock<Environment>,
) -> Result<Value, Error> {
	interpret_expr(&expr.val.expression, env).await
}
