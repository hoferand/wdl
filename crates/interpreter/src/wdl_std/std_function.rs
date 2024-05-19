use futures::future::BoxFuture;

use crate::{wdl_std::CallContext, Error, Value};

pub trait StdFunction {
	#[allow(unused)]
	fn clone_box(&self) -> Box<dyn StdFunction>;
	fn call_with_ctx(&self, ctx: CallContext, strict: bool) -> BoxFuture<Result<Value, Error>>;
}
