use std::vec::IntoIter;

use crate::{Error, Value};

pub(crate) trait StdFunction {
	fn clone_box(&self) -> Box<dyn StdFunction>;
	fn call_with_args(&self, args: &mut IntoIter<Value>) -> Result<Value, Error>;
}
