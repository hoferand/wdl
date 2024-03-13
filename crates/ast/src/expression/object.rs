use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Expression;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
	pub values: HashMap<String, Expression>,
}
