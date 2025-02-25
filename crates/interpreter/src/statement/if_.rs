use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Else, If, Node};

use crate::{
	Environment, Error, Interrupt, Scope, expression::interpret_expression,
	statement::interpret_block,
};

#[async_recursion]
pub async fn interpret_if(
	stmt: &Node<If>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	if interpret_expression(&stmt.val.condition, scope, env)
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
