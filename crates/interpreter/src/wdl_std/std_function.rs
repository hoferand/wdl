use futures::future::BoxFuture;

use crate::{Error, Value, wdl_std::CallContext};

pub trait StdFunction {
	#[allow(unused)]
	fn clone_box(&self) -> Box<dyn StdFunction>;
	fn call_with_ctx(&self, ctx: CallContext, strict: bool) -> BoxFuture<Result<Value, Error>>;
}
