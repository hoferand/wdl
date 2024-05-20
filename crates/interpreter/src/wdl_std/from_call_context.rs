use std::sync::Arc;

use ast::{Identifier, Span};

use crate::{
	wdl_std::{name, Arg, CallContext, FromValue},
	Environment, Error, ErrorKind, Value,
};

pub trait FromCallContext: Sized {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error>;
}

impl<T: FromValue, const N: u32> FromCallContext for Arg<T, N> {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error> {
		if let Some(val) = Option::<Arg<T, N>>::from_ctx(ctx)? {
			Ok(val)
		} else {
			let id = Identifier {
				id: String::from_utf8_lossy(name(N)).into_owned(),
			};
			Err(Error {
				kind: ErrorKind::MissingArgument { id },
				span: Some(ctx.fn_span),
			})
		}
	}
}

impl<T: FromValue, const N: u32> FromCallContext for Option<Arg<T, N>> {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error> {
		let id = Identifier {
			id: String::from_utf8_lossy(name(N)).into_owned(),
		};

		let mut arg = ctx.args.next();

		if arg.is_none() {
			arg = ctx.named_args.get(&id).cloned();
			ctx.named_args.remove(&id);
		}

		if let Some(arg) = arg {
			let is_null = arg.val == Value::Null;
			let arg_clone = arg.clone();
			let value = T::from_value(arg.val).map_err(|mut err| {
				err.span = Some(arg.span);
				err
			})?;
			if let Some(val) = value {
				Ok(Some(Arg::new(arg_clone.idx, arg_clone.span, val)))
			} else if is_null {
				return Ok(None);
			} else {
				Err(Error {
					kind: ErrorKind::InvalidType {
						msg: format!(
							"expected `{}`, given `{}` for argument `{}` on index {}",
							T::get_type(),
							arg_clone.val.get_type(),
							id,
							arg_clone.idx
						),
					},
					span: Some(arg_clone.span),
				})
			}
		} else {
			Ok(None)
		}
	}
}

// TODO: implement for Arg<Vec<T>, N> and Arg<HashMap<String, T>, N>

impl FromCallContext for Arc<Environment> {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error> {
		Ok(Arc::clone(&ctx.env))
	}
}

impl FromCallContext for Span {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error> {
		Ok(ctx.fn_span)
	}
}
