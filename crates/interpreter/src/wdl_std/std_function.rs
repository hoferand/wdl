use std::sync::Arc;

use futures::future::BoxFuture;

use crate::{Environment, Error, Value};

use super::Arguments;

pub trait StdFunction {
	fn clone_box(&self) -> Box<dyn StdFunction>;
	fn call_with_args(
		&self,
		env: Arc<Environment>,
		args: Arguments,
	) -> BoxFuture<Result<Value, Error>>;
}
