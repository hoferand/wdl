mod order;
pub use order::interpret_order;
mod block;
use block::interpret_block;
mod par;
use par::interpret_par;
mod if_;
use if_::interpret_if;
mod while_;
use while_::interpret_while;
mod print;
use print::interpret_print;
mod break_;
use break_::interpret_break;
mod continue_;
use continue_::interpret_continue;
mod return_;
use return_::interpret_return;
mod sleep;
use sleep::interpret_sleep;
mod function_declaration;
use function_declaration::interpret_function_declaration;
mod global_declaration;
use global_declaration::interpret_global_declaration;
mod import;
use import::interpret_import;
mod let_;
use let_::interpret_let;

use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::Statement;

use crate::{Environment, Error, Interrupt};

use super::expr::interpret_expr;

#[async_recursion]
pub async fn interpret_stmt(
	stmt: &Statement,
	env: &RwLock<Environment>,
) -> Result<Interrupt, Error> {
	match stmt {
		Statement::Expression(expr) => {
			interpret_expr(expr, env).await?;
			Ok(Interrupt::None)
		}
		Statement::Block(block) => interpret_block(block, env).await,
		Statement::Break(_) => interpret_break(),
		Statement::Continue(_) => interpret_continue(),
		Statement::FunctionDeclaration(fn_) => interpret_function_declaration(fn_, env).await,
		Statement::GlobalDeclaration(global) => interpret_global_declaration(global, env).await,
		Statement::If(if_) => interpret_if(if_, env).await,
		Statement::Import(import) => interpret_import(import, env).await,
		Statement::Let(let_) => interpret_let(let_, env).await,
		Statement::Order(order) => interpret_order(order, env).await,
		Statement::Par(par) => {
			interpret_par(par, env).await?;
			Ok(Interrupt::None)
		}
		Statement::Print(print) => {
			interpret_print(print, env).await?;
			Ok(Interrupt::None)
		}
		Statement::Return(return_) => interpret_return(return_, env).await,
		Statement::Sleep(sleep) => {
			interpret_sleep(sleep, env).await?;
			Ok(Interrupt::None)
		}
		Statement::While(while_) => interpret_while(while_, env).await,
	}
}
