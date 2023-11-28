mod order;
pub use order::interpret_order;
mod block;
pub use block::interpret_block;
mod par;
use par::interpret_par;
mod if_;
use if_::interpret_if;
mod while_;
use while_::interpret_while;
mod break_;
use break_::interpret_break;
mod continue_;
use continue_::interpret_continue;
mod return_;
use return_::interpret_return;
mod function_declaration;
pub use function_declaration::interpret_function_declaration;
mod global_declaration;
pub use global_declaration::interpret_global_declaration;
mod let_;
use let_::interpret_let;

use async_recursion::async_recursion;

use ast::Statement;

use crate::{Environment, Error, Interrupt};

use super::expr::interpret_expr;

#[async_recursion]
pub async fn interpret_stmt(
	stmt: &Statement,
	env: &Environment,
	g_env: &Environment,
) -> Result<Interrupt, Error> {
	match stmt {
		Statement::Expression(expr) => {
			interpret_expr(expr, env, g_env).await?;
			Ok(Interrupt::None)
		}
		Statement::Block(block) => interpret_block(block, env, g_env).await,
		Statement::Break(_) => interpret_break(),
		Statement::Continue(_) => interpret_continue(),
		Statement::If(if_) => interpret_if(if_, env, g_env).await,
		Statement::Let(let_) => interpret_let(let_, env, g_env).await,
		Statement::Par(par) => {
			interpret_par(par, env, g_env).await?;
			Ok(Interrupt::None)
		}
		Statement::Return(return_) => interpret_return(return_, env, g_env).await,
		Statement::While(while_) => interpret_while(while_, env, g_env).await,
	}
}
