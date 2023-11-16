use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{GlobalDeclaration, Node};

use crate::{Environment, Error, Interrupt};

#[async_recursion]
pub async fn interpret_global_declaration(
	stmt: &Node<GlobalDeclaration>,
	env: &RwLock<Environment>,
) -> Result<Interrupt, Error> {
	todo!()
}
