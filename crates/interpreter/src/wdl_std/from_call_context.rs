use serde::Deserialize;

use crate::{ChannelId, Error, ErrorKind, Value};

use super::CallContext;

pub(crate) trait FromCallContext: Sized {
	fn from_ctx(args: &mut CallContext) -> Result<Self, Error>;
}

impl<T> FromCallContext for T
where
	T: for<'de> Deserialize<'de>,
{
	fn from_ctx(args: &mut CallContext) -> Result<Self, Error> {
		if let Some(arg) = args.args.next() {
			let json_val = match serde_json::to_value(arg.val) {
				Ok(val) => val,
				Err(err) => {
					return Err(Error {
						kind: ErrorKind::InvalidType {
							msg: err.to_string(),
						},
						src: Some(arg.span),
					});
				}
			};

			let rust_val = match serde_json::from_value(json_val) {
				Ok(val) => val,
				Err(err) => {
					return Err(Error {
						kind: ErrorKind::InvalidType {
							msg: err.to_string(),
						},
						src: Some(arg.span),
					});
				}
			};
			Ok(rust_val)
		} else {
			Err(Error {
				kind: ErrorKind::TooFewArguments,
				src: Some(args.fn_span.clone()),
			})
		}
	}
}

impl FromCallContext for ChannelId {
	fn from_ctx(args: &mut CallContext) -> Result<Self, Error> {
		if let Some(arg) = args.args.next() {
			if let Value::Channel(ch_id) = arg.val {
				Ok(ch_id)
			} else {
				Err(Error {
					kind: ErrorKind::InvalidType {
						msg: format!("Expected channel but got `{}`", arg.val.get_type()),
					},
					src: Some(arg.span),
				})
			}
		} else {
			Err(Error {
				kind: ErrorKind::TooFewArguments,
				src: Some(args.fn_span.clone()),
			})
		}
	}
}
