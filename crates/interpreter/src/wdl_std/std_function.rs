use crate::{Error, Value};

use super::Arguments;

pub(crate) trait StdFunction {
	fn clone_box(&self) -> Box<dyn StdFunction>;
	fn call_with_args(&self, args: &mut Arguments) -> Result<Value, Error>;
}
