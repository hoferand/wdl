use std::sync::Arc;

use futures::future::BoxFuture;

use crate::{environment::Environment, Error, Value};

use super::{Arguments, StdFunction};

pub(crate) struct HandlerFunction<H: Clone> {
	pub handler: H,
	pub call: fn(H, Arc<Environment>, Arguments) -> BoxFuture<'static, Result<Value, Error>>,
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

	fn call_with_args(
		&self,
		env: Arc<Environment>,
		args: Arguments,
	) -> BoxFuture<Result<Value, Error>> {
		(self.call)(self.handler.clone(), env, args)
	}
}
