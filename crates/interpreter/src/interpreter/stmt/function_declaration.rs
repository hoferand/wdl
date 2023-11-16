use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{FunctionDeclaration, Node};

use crate::{Environment, Error, Interrupt};

#[async_recursion]
pub async fn interpret_function_declaration(
	_stmt: &Node<FunctionDeclaration>,
	_env: &RwLock<Environment>,
) -> Result<Interrupt, Error> {
	todo!()
}
