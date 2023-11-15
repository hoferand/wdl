use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Logical, LogicalOperator, Node};

use crate::{environment::Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_logical(
	expr: &Node<Logical>,
	env: &RwLock<Environment>,
) -> Result<Value, Error> {
	let left = interpret_expr(&expr.val.left, env).await?;

	// short circuit evaluation
	match (left.boolify(), &expr.val.op.val) {
		(false, LogicalOperator::And) => Ok(Value::Bool(false)),
		(true, LogicalOperator::And) => interpret_expr(&expr.val.right, env).await,
		(false, LogicalOperator::Or) => interpret_expr(&expr.val.right, env).await,
		(true, LogicalOperator::Or) => Ok(Value::Bool(true)),
	}
}
