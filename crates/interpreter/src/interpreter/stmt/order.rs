use async_recursion::async_recursion;

use ast::{Node, Order};

use crate::{Environment, Error, Interrupt};

use super::interpret_block;

#[async_recursion]
pub async fn interpret_order(stmt: &Node<Order>, env: &Environment) -> Result<Interrupt, Error> {
	let ret = interpret_block(&stmt.val.block, env).await?;
	if !ret.is_none() {
		return Err(Error::Fatal(format!(
			"AST invalid, `{}` in order block found",
			ret.get_type()
		)));
	}

	Ok(Interrupt::None)
}
