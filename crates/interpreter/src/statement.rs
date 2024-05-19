use std::sync::Arc;

use async_recursion::async_recursion;

use ast::Statement;

use crate::{expression::interpret_expression, Environment, Error, Interrupt, Scope};

pub mod block;
pub use block::interpret_block;

mod assignment;
use assignment::interpret_assignment;
mod break_;
use break_::interpret_break;
mod continue_;
use continue_::interpret_continue;
mod if_;
use if_::interpret_if;
mod let_;
use let_::interpret_let;
mod return_;
use return_::interpret_return;
mod send;
use send::interpret_send;
mod while_;
use while_::interpret_while;

#[async_recursion]
pub async fn interpret_statement(
	stmt: &Statement,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	match stmt {
		Statement::Assignment(stmt) => interpret_assignment(stmt, scope, env).await,
		Statement::Expression(expr) => {
			interpret_expression(expr, scope, env).await?;
			Ok(Interrupt::None)
		}
		Statement::Block(block) => interpret_block(block, scope, env).await,
		Statement::Break(_) => interpret_break(),
		Statement::Continue(_) => interpret_continue(),
		Statement::If(if_) => interpret_if(if_, scope, env).await,
		Statement::Let(let_) => interpret_let(let_, scope, env).await,
		Statement::Return(return_) => interpret_return(return_, scope, env).await,
		Statement::Send(stmt) => interpret_send(stmt, scope, env).await,
		Statement::While(while_) => interpret_while(while_, scope, env).await,
	}
}
