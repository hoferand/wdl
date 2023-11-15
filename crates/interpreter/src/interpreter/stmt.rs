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
		Statement::If(if_) => interpret_if(if_, env).await,
		Statement::Let(_) => todo!(),
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
