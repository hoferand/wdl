use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Assignment, Node, Span};

use crate::{Environment, Error, Interrupt, Scope};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_assignment(
	expr: &Node<Span, Assignment<Span>>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let value = interpret_expr(&expr.val.value, scope, env).await?;
	let id = expr.val.id.clone();
	scope.assign(id, value.clone()).await?;

	Ok(Interrupt::None)
}
