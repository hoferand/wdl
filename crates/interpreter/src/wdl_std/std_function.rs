use futures::future::BoxFuture;

use crate::{Arguments, Error, Value};

pub(crate) trait StdFunction {
	fn clone_box(&self) -> Box<dyn StdFunction>;
	fn call_with_args(&self, args: Arguments) -> BoxFuture<Result<Value, Error>>;
}
