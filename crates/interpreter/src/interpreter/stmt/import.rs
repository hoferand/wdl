use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Import, Node};

use crate::{Environment, Error, Interrupt};

#[async_recursion]
pub async fn interpret_import(
	stmt: &Node<Import>,
	env: &RwLock<Environment>,
) -> Result<Interrupt, Error> {
	todo!()
}
