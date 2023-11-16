mod global_declaration;
pub(crate) use global_declaration::parse_global_declaration;
mod order;
pub(crate) use order::parse_order;
mod block;
pub(crate) use block::parse_block;
mod return_;
pub(crate) use return_::parse_return;
mod break_;
pub(crate) use break_::parse_break;
mod continue_;
pub(crate) use continue_::parse_continue;
mod while_;
pub(crate) use while_::parse_while;
mod else_;
pub(crate) use else_::parse_else;
mod if_;
pub(crate) use if_::parse_if;
mod print;
pub(crate) use print::parse_print;
mod sleep;
pub(crate) use sleep::parse_sleep;
mod par;
pub(crate) use par::parse_par;
mod let_;
pub(crate) use let_::parse_let;
mod function_declaration;
pub(crate) use function_declaration::parse_function_declaration;

use ast::Statement;

use crate::{Parser, ParserError, TokenValue};

use super::expression::parse_expression;

pub(crate) fn parse_statement(parser: &mut Parser) -> Result<Option<Statement>, ParserError> {
	let Some(token) = parser.tokens.peek() else {
		return Ok(None);
	};

	Ok(Some(match token.value {
		TokenValue::EoF => return Ok(None),

		// statements
		TokenValue::Global | TokenValue::At => {
			Statement::GlobalDeclaration(parse_global_declaration(parser)?)
		}
		TokenValue::Order => Statement::Order(parse_order(parser)?),
		TokenValue::Fn => Statement::FunctionDeclaration(parse_function_declaration(parser)?),
		TokenValue::Let => Statement::Let(parse_let(parser)?),
		TokenValue::Par => Statement::Par(parse_par(parser)?),
		TokenValue::Sleep => Statement::Sleep(parse_sleep(parser)?),
		TokenValue::Print => Statement::Print(parse_print(parser)?),
		TokenValue::If => Statement::If(parse_if(parser)?),
		TokenValue::While => Statement::While(parse_while(parser)?),
		TokenValue::Continue => Statement::Continue(parse_continue(parser)?),
		TokenValue::Break => Statement::Break(parse_break(parser)?),
		TokenValue::Return => Statement::Return(parse_return(parser)?),

		// expression
		_ => Statement::Expression(parse_expression(parser)?),
	}))
}
