use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Assignment, Node};

use crate::{expression::interpret_expression, Environment, Error, Interrupt, Scope};

#[async_recursion]
pub async fn interpret_assignment(
	expr: &Node<Assignment>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let value = interpret_expression(&expr.val.value, scope, env).await?;
	let id = expr.val.id.clone();
	scope.assign(id, value.clone()).await?;

	Ok(Interrupt::None)
}
