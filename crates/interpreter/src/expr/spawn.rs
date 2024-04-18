use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Span, Spawn};
use logger::error;
use logger::Colorize;

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_spawn(
	expr: &Node<Span, Spawn<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Value, Error> {
	let (ch_id, ch) = g_env.create_ch(1).await;

	let ch_async = ch.clone();
	let expr_async = expr.val.expr.clone();
	let env_async = env.clone();
	let g_env_async = g_env.clone();
	tokio::spawn(async move {
		match interpret_expr(&expr_async, &env_async, &g_env_async).await {
			Ok(value) => ch_async.send(value).await,
			Err(err) => {
				error!("{:?}", err);
				ch_async.send(Value::Null).await
			}
		};
	});

	Ok(Value::Channel(ch_id))
}
