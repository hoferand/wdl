use std::vec::IntoIter;

use crate::{Error, Value};

use super::{FromArguments, IntoResult};

pub trait Handler<T> {
	fn call(&self, args: &mut IntoIter<Value>) -> Result<Value, Error>;
}

impl_handler!();
impl_handler!(T1);
impl_handler!(T1, T2);
impl_handler!(T1, T2, T3);

macro_rules! impl_handler {
	($($ty:ident),*) => {
		impl<F, $($ty,)* R> Handler<($($ty,)*)> for F
		where
			F: Fn($($ty,)*) -> R,
			$($ty: FromArguments,)*
			R: IntoResult
		{
      #[allow(unused_variables)]
			fn call(&self, args: &mut IntoIter<Value>) -> Result<Value, Error> {
				(self)($($ty::from_args(args)?,)*).into_result()
			}
		}
	}
}

use impl_handler;
