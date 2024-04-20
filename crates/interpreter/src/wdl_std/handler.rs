use std::future::Future;

use futures::future::BoxFuture;

use crate::{Error, Value};

use super::{CallContext, FromCallContext, IntoResult};

pub trait Handler<T>: Clone + Send + Sized + 'static {
	fn call(self, ctx: CallContext) -> BoxFuture<'static, Result<Value, Error>>;
}

impl_handler!();
impl_handler!(T1);
impl_handler!(T1, T2);
impl_handler!(T1, T2, T3);
impl_handler!(T1, T2, T3, T4);

macro_rules! impl_handler {
	($($ty:ident),*) => {
		impl<F, Fut, $($ty,)* R> Handler<($($ty,)*)> for F
		where
			F: FnOnce($($ty,)*) -> Fut + Clone + Send + 'static, // TODO: check FnOnce
			Fut: Future<Output = R> + Send,
			$($ty: FromCallContext + Send,)*
			R: IntoResult
		{
			#[allow(non_snake_case, unused_variables, unused_mut)]
			fn call(self, mut ctx: CallContext) -> BoxFuture<'static, Result<Value, Error>> {
				Box::pin(async move {
					// TODO: check for unused arguments
					(self)($($ty::from_ctx(&mut ctx)?,)*).await.into_result()
				})
			}
		}
	}
}

use impl_handler;
