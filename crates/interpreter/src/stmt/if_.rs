use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Else, If, Node, Span};

use crate::{Environment, Error, Interrupt, Scope};

use super::{interpret_block, interpret_expr};

#[async_recursion]
pub async fn interpret_if(
	stmt: &Node<Span, If<Span>>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	if interpret_expr(&stmt.val.condition, scope, env)
		.await?
		.boolify()
	{
		interpret_block(&stmt.val.then, scope, env).await
	} else if let Some(else_) = &stmt.val.else_ {
		match &else_.val {
			Else::Else(block) => interpret_block(block, scope, env).await,
			Else::ElseIf(else_if) => interpret_if(else_if, scope, env).await,
		}
	} else {
		Ok(Interrupt::None)
	}
}
