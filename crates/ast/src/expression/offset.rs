use serde::{Deserialize, Serialize};

use crate::{Expression, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offset<S: Source> {
	pub value: Box<Expression<S>>,
	pub offset: Box<Expression<S>>,
}
