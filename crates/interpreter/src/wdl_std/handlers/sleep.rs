use std::time::Duration;

use futures::future::BoxFuture;

use crate::{Error, Value};

pub fn sleep(val: Value) -> BoxFuture<'static, Result<Value, Error>> {
	let ms = match val {
		Value::Number(n) => Duration::from_millis(n as u64),
		_ => Duration::ZERO,
	};

	Box::pin(async move {
		tokio::time::sleep(ms).await;
		Ok(Value::Null)
	})
}

/*
pub async fn sleep(val: Value) {
	let ms = match val {
		Value::Number(n) => Duration::from_millis(n as u64),
		_ => Duration::ZERO,
	};

	tokio::time::sleep(ms).await;
}
*/
