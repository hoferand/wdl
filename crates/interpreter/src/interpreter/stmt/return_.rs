use async_recursion::async_recursion;

use ast::{Node, Return};

use crate::{Environment, Error, Interrupt};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_return(stmt: &Node<Return>, env: &Environment) -> Result<Interrupt, Error> {
	let val = interpret_expr(&stmt.val.value, env).await?;

	Ok(Interrupt::Return(val))
}
