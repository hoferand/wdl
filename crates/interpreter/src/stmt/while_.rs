use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, While};

use crate::{Environment, Error, Interrupt};

use super::{interpret_block, interpret_expr};

#[async_recursion]
pub async fn interpret_while(
	while_: &Node<While>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	while interpret_expr(&while_.val.condition, env, g_env)
		.await?
		.boolify()
	{
		match interpret_block(&while_.val.do_, env, g_env).await? {
			Interrupt::None | Interrupt::Continue => {}
			Interrupt::Break => break,
			ret @ Interrupt::Return(_) => return Ok(ret),
		}
	}

	Ok(Interrupt::None)
}
