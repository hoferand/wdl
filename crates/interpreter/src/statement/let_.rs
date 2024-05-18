use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Let, Node};

use crate::{expression::interpret_expression, Environment, Error, Interrupt, Scope};

#[async_recursion]
pub async fn interpret_let(
	stmt: &Node<Let>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let value = interpret_expression(&stmt.val.value, scope, env).await?;
	let id = stmt.val.id.clone();
	scope.declare(id, value).await?;

	Ok(Interrupt::None)
}
