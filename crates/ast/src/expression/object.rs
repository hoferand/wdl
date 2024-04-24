use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Expression, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Object<S: Source> {
	pub values: HashMap<String, Expression<S>>,
}
