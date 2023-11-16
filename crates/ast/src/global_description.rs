use crate::Node;

#[derive(Debug, Clone)]
pub struct GlobalDescription {
	pub type_: Node<String>,
	pub name: Node<String>,
}
