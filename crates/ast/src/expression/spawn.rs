use serde::{Deserialize, Serialize};

use crate::{Expression, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Spawn<S: Source> {
	pub expr: Box<Expression<S>>,
}
