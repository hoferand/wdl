use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Expression;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Object {
	pub values: HashMap<String, Expression>,
}
