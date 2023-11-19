use async_recursion::async_recursion;

use ast::{Logical, LogicalOperator, Node};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_logical(
	expr: &Node<Logical>,
	env: &Environment,
	g_env: &Environment,
) -> Result<Value, Error> {
	let left = interpret_expr(&expr.val.left, env, g_env).await?;

	// short circuit evaluation
	match (left.boolify(), &expr.val.op.val) {
		(false, LogicalOperator::And) => Ok(Value::Bool(false)),
		(true, LogicalOperator::And) => interpret_expr(&expr.val.right, env, g_env).await,
		(false, LogicalOperator::Or) => interpret_expr(&expr.val.right, env, g_env).await,
		(true, LogicalOperator::Or) => Ok(Value::Bool(true)),
	}
}
