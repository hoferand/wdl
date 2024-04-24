use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::Source;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Node<S: Source, V: Debug + Clone> {
	// TODO: add Serialize + Deserialize bound to S and V
	pub src: S,
	pub val: V,
}
