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
		Value::Channel(ch_id) => {
			let Some(ch) = g_env.get_ch(&ch_id).await else {
				return Err(Error::fatal(format!("Channel `{}` not found", ch_id.id)));
			};
			let value = interpret_expr(&expr.val.value, env, g_env).await?;
			if ch.send(value).await.is_none() {
				Err(Error::fatal("Cannot send on closed channel".to_owned()))
			} else {
				Ok(Interrupt::None)
			}
		}
		val => Err(Error::fatal(format!(
			"Cannot send on type `{}`",
			val.get_type()
		))),
	}
}
