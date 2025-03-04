use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, While};

use crate::{
	Environment, Error, Interrupt, Scope, expression::interpret_expression,
	statement::interpret_block,
};

#[async_recursion]
pub async fn interpret_while(
	while_: &Node<While>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	while interpret_expression(&while_.val.condition, scope, env)
		.await?
		.boolify()
	{
		match interpret_block(&while_.val.do_, scope, env).await? {
			Interrupt::None | Interrupt::Continue => {}
			Interrupt::Break => break,
			ret @ Interrupt::Return(_) => return Ok(ret),
		}

		#[cfg(feature = "playground")]
		{
			tokio::time::sleep(std::time::Duration::from_millis(500)).await; // to reduce damage of infinite loops
		}
	}

	Ok(Interrupt::None)
}
