use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Return, Span};

use crate::{Environment, Error, Interrupt, Scope};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_return(
	stmt: &Node<Span, Return<Span>>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let val = interpret_expr(&stmt.val.value, scope, env).await?;

	Ok(Interrupt::Return(val))
}
