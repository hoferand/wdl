use serde::{Deserialize, Serialize};

use crate::{Expression, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Array<S: Source> {
	pub values: Vec<Expression<S>>,
}
