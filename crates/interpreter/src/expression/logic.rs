use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Logic, LogicOperator, Node};

use crate::{Environment, Error, Scope, Value, expression::interpret_expression};

#[async_recursion]
pub async fn interpret_logic(
	expr: &Node<Logic>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	let left = interpret_expression(&expr.val.left, scope, env).await?;

	// short circuit evaluation
	match (left.boolify(), &expr.val.op.val) {
		(false, LogicOperator::And) => Ok(Value::Bool(false)),
		(true, LogicOperator::Or) => Ok(Value::Bool(true)),
		_ => Ok(Value::Bool(
			interpret_expression(&expr.val.right, scope, env)
				.await?
				.boolify(),
		)),
	}
}
