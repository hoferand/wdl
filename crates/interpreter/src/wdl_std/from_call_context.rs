use std::sync::Arc;

use ast::Identifier;

use crate::{Error, ErrorKind, Value};

use super::{name, Arg, CallContext, Env, FromValue};

pub(crate) trait FromCallContext: Sized {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error>;
}

impl<T: FromValue, const N: u32> FromCallContext for Arg<T, N> {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error> {
		if let Some(val) = Option::<Arg<T, N>>::from_ctx(ctx)? {
			Ok(val)
		} else {
			let id = Identifier(String::from_utf8_lossy(name(N)).into_owned());
			Err(Error {
				kind: ErrorKind::MissingArgument { id },
				src: Some(ctx.fn_span.clone()),
			})
		}
	}
}

impl<T: FromValue, const N: u32> FromCallContext for Option<Arg<T, N>> {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error> {
		let id = Identifier(String::from_utf8_lossy(name(N)).into_owned());

		let mut arg = ctx.args.next();

		if arg.is_none() {
			arg = ctx.named_args.get(&id).cloned();
			ctx.named_args.remove(&id);
		}

		if let Some(arg) = arg {
			if arg.val == Value::Null {
				return Ok(None);
			}
			let arg_clone = arg.clone();
			let value = T::from_value(arg.val).map_err(|mut err| {
				err.src = Some(arg.span);
				err
			})?;
			if let Some(val) = value {
				Ok(Some(Arg::new(arg_clone.idx, arg_clone.span, val)))
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
					src: Some(arg_clone.span),
				})
			}
		} else {
			Ok(None)
		}
	}
}

// TODO: implement for Arg<Vec<T>, N> and Arg<HashMap<String, T>, N>

impl FromCallContext for Env {
	fn from_ctx(ctx: &mut CallContext) -> Result<Self, Error> {
		Ok(Env(Arc::clone(&ctx.env)))
	}
}
