use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Let, Node};

use crate::{Environment, Error, Interrupt};

#[async_recursion]
pub async fn interpret_let(
	_stmt: &Node<Let>,
	_env: &RwLock<Environment>,
) -> Result<Interrupt, Error> {
	todo!()
}
