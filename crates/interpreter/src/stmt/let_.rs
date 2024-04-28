use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Let, Node, Span};

use crate::{expr::interpret_expr, Environment, Error, Interrupt, Scope};

#[async_recursion]
pub async fn interpret_let(
	stmt: &Node<Span, Let<Span>>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let value = interpret_expr(&stmt.val.value, scope, env).await?;
	let id = stmt.val.id.clone();
	scope.declare(id, value).await?;

	Ok(Interrupt::None)
}
