use std::future::Future;

use futures::future::BoxFuture;

use crate::{Error, ErrorKind, Value};

use super::{CallContext, FromCallContext, IntoResult};

pub trait Handler<T>: Clone + Send + Sized + 'static {
	fn call(self, ctx: CallContext, strict: bool) -> BoxFuture<'static, Result<Value, Error>>;
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
			fn call(self, mut ctx: CallContext, strict: bool) -> BoxFuture<'static, Result<Value, Error>> {
				Box::pin(async move {
					let mut cnt = 0;
					$(
						let $ty = $ty::from_ctx(&mut ctx)?; // TODO: do not shadow type parameter
						cnt += 1;
					)*

					let rem = ctx.args.count();
					let rem_named = ctx.named_args.keys().len();

					if let Some((id, arg)) = ctx.named_args.into_iter().next() {
						return Err(Error{
							kind: ErrorKind::UnknownArgument {
								id
							},
							src: Some(arg.span)
						});
					}

					if strict && rem != 0 {
						return Err(Error {
							kind: ErrorKind::ArityMismatch {
								expected: cnt,
								given: cnt + rem + rem_named
							},
							src: Some(ctx.fn_span)
						});
					}

					(self)($($ty,)*).await.into_result()
				})
			}
		}
	}
}

use impl_handler;
