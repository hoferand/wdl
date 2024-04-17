use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Send, Span};

use crate::{interrupt::Interrupt, Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_send(
	expr: &Node<Span, Send<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	match interpret_expr(&expr.val.ch, env, g_env).await? {
		// TODO: improve error messages
		Value::Channel(ch) => {
			let value = interpret_expr(&expr.val.value, env, g_env).await?;
			if ch.send(value).await.is_none() {
				Err(Error::Fatal("Cannot send on closed channel".to_owned()))
			} else {
				Ok(Interrupt::None)
			}
		}
		val => Err(Error::Fatal(format!(
			"Cannot send on type `{}`",
			val.get_type()
		))),
	}
}
