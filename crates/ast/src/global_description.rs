use serde::{Deserialize, Serialize};

use crate::Node;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalDescription {
	pub type_: Node<String>,
	pub name: Node<String>,
}
