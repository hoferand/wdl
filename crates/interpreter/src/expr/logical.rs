use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Logical, LogicalOperator, Node, Span};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_logical(
	expr: &Node<Span, Logical<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Value, Error> {
	let left = interpret_expr(&expr.val.left, env, g_env).await?;

	// short circuit evaluation
	match (left.boolify(), &expr.val.op.val) {
		(false, LogicalOperator::And) => Ok(Value::Bool(false)),
		(true, LogicalOperator::Or) => Ok(Value::Bool(true)),
		_ => interpret_expr(&expr.val.right, env, g_env).await,
	}
}
