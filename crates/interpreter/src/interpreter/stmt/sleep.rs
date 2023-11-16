use std::time::Duration;

use async_recursion::async_recursion;
use tokio::{sync::RwLock, time::sleep};

use ast::{Node, Sleep};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_sleep(stmt: &Node<Sleep>, env: &RwLock<Environment>) -> Result<(), Error> {
	let time = interpret_expr(&stmt.val.time, env).await?;

	if let Value::Number(millis) = time {
		sleep(Duration::from_millis(millis as u64)).await;
		Ok(())
	} else {
		Err(Error::Fatal(format!(
			"Invalid type, sleep `{}`",
			time.get_type()
		)))
	}
}
