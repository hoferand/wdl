use serde::{Deserialize, Serialize};

use crate::{Expression, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Send<S: Source> {
	pub ch: Box<Expression<S>>,
	pub value: Box<Expression<S>>,
}
