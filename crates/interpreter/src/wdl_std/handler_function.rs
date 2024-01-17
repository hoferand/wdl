use futures::future::BoxFuture;

use crate::{Arguments, Error, Value};

use super::StdFunction;

pub(crate) struct HandlerFunction<H: Clone> {
	pub handler: H,
	pub call: fn(H, Arguments) -> BoxFuture<'static, Result<Value, Error>>,
}

impl<H: Clone> Clone for HandlerFunction<H> {
	fn clone(&self) -> Self {
		Self {
			handler: self.handler.clone(),
			call: self.call,
		}
	}
}

impl<H> StdFunction for HandlerFunction<H>
where
	H: Clone + 'static,
{
	fn clone_box(&self) -> Box<dyn StdFunction> {
		Box::new(self.clone())
	}

	fn call_with_args(&self, args: Arguments) -> BoxFuture<Result<Value, Error>> {
		(self.call)(self.handler.clone(), args)
	}
}
