use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Else, If, Node};

use crate::{Environment, Error, Interrupt};

use super::{interpret_block, interpret_expr};

#[async_recursion]
pub async fn interpret_if(
	stmt: &Node<If>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	if interpret_expr(&stmt.val.condition, env, g_env)
		.await?
		.boolify()
	{
		interpret_block(&stmt.val.then, env, g_env).await
	} else if let Some(else_) = &stmt.val.else_ {
		match &else_.val {
			Else::Else(block) => interpret_block(block, env, g_env).await,
			Else::ElseIf(else_if) => interpret_if(else_if, env, g_env).await,
		}
	} else {
		Ok(Interrupt::None)
	}
}
