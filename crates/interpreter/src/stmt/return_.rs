use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Return};

use crate::{Environment, Error, Interrupt, Scope};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_return(
	stmt: &Node<Return>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let val = interpret_expr(&stmt.val.value, scope, env).await?;

	Ok(Interrupt::Return(val))
}
