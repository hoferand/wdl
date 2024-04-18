use std::sync::Arc;

use crate::{Error, ErrorKind};

use super::{CallContext, Env, FromArgument};

pub(crate) trait FromCallContext: Sized {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error>;
}

impl<T: FromArgument> FromCallContext for T {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error> {
		if let Some(arg) = ctx.args.next() {
			Ok(T::from_arg(arg)?)
		} else {
			Err(Error {
				kind: ErrorKind::TooFewArguments,
				src: Some(ctx.fn_span.clone()),
			})
		}
	}
}

impl<T: FromArgument> FromCallContext for Option<T> {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error> {
		if let Some(arg) = ctx.args.next() {
			Ok(Some(T::from_arg(arg)?))
		} else {
			Ok(None)
		}
	}
}

impl FromCallContext for Env {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error> {
		Ok(Env(Arc::clone(&ctx.env)))
	}
}
