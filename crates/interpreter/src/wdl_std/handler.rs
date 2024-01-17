use std::future::Future;

use futures::future::BoxFuture;

use crate::{Error, Value};

use super::{Arguments, FromArguments, IntoResult};

pub trait Handler<T>: Clone + Send + Sized + 'static {
	fn call(self, args: Arguments) -> BoxFuture<'static, Result<Value, Error>>;
}

impl_handler!();
impl_handler!(T1);
impl_handler!(T1, T2);
impl_handler!(T1, T2, T3);

macro_rules! impl_handler {
	($($ty:ident),*) => {
		impl<F, Fut, $($ty,)* R> Handler<($($ty,)*)> for F
		where
			F: FnOnce($($ty,)*) -> Fut + Clone + Send + 'static,
			Fut: Future<Output = R> + Send,
			$($ty: FromArguments + Send,)*
			R: IntoResult
		{
	  #[allow(non_snake_case, unused_variables, unused_mut)]
			fn call(self, mut args: Arguments) -> BoxFuture<'static, Result<Value, Error>> {
				Box::pin(async move {
					(self)($($ty::from_args(&mut args)?,)*).await.into_result()
				})
			}
		}
	}
}

use impl_handler;
