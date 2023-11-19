use std::time::Duration;

use async_recursion::async_recursion;
use tokio::time::sleep;

use ast::{Node, Sleep};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_sleep(
	stmt: &Node<Sleep>,
	env: &Environment,
	g_env: &Environment,
) -> Result<(), Error> {
	let time = interpret_expr(&stmt.val.time, env, g_env).await?;

	if let Value::Number(millis) = time {
		sleep(Duration::from_millis(millis as u64)).await;
		Ok(())
	} else {
		Err(Error::InvalidType {
			msg: format!("sleep `{}`", time.get_type()),
			span: stmt.val.time.get_span().clone(),
		})
	}
}
