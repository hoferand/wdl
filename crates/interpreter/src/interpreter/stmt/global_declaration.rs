use async_recursion::async_recursion;

use ast::{GlobalDeclaration, Node};

use crate::{Environment, Error, Interrupt};

#[async_recursion]
pub async fn interpret_global_declaration(
	_stmt: &Node<GlobalDeclaration>,
	_env: &Environment,
) -> Result<Interrupt, Error> {
	todo!()
}
