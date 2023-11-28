use futures::future::BoxFuture;

use crate::{Error, Value};

pub fn print(val: Value) -> BoxFuture<'static, Result<Value, Error>> {
	println!("{}", val.to_string());

	Box::pin(async { Ok(Value::Null) })
}

/*
TODO:
pub fn print(val: Value) {
	println!("{}", val.to_string());
}
*/
