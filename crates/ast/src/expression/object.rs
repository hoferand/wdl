use std::collections::HashMap;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct Object {
	pub values: HashMap<String, Expression>,
}
