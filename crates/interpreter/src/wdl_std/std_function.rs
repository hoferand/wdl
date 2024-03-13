use futures::future::BoxFuture;

use crate::{Error, Value};

use super::Arguments;

pub trait StdFunction {
	fn clone_box(&self) -> Box<dyn StdFunction>;
	fn call_with_args(&self, args: Arguments) -> BoxFuture<Result<Value, Error>>;
}
