use futures::future::BoxFuture;

use crate::{Error, Value};

use super::CallContext;

pub trait StdFunction {
	fn clone_box(&self) -> Box<dyn StdFunction>;
	fn call_with_ctx(&self, ctx: CallContext, strict: bool) -> BoxFuture<Result<Value, Error>>;
}
