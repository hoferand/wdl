use async_recursion::async_recursion;

use ast::{Node, Order};

use crate::{Environment, Error, Interrupt};

use super::interpret_block;

#[async_recursion]
pub async fn interpret_order(stmt: &Node<Order>, env: &Environment) -> Result<Interrupt, Error> {
	if !interpret_block(&stmt.val.block, env).await?.is_none() {
		return Err(Error::Fatal(
			"AST invalid, interrupt in order block found".to_owned(),
		));
	}

	Ok(Interrupt::None)
}
