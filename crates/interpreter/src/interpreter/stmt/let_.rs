use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Let, Node};

use crate::{Environment, Error, Interrupt};

#[async_recursion]
pub async fn interpret_let(
	stmt: &Node<Let>,
	env: &RwLock<Environment>,
) -> Result<Interrupt, Error> {
	todo!()
}
