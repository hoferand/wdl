use ast::{Assignment, Expression, Node, Send, Span, Statement};

use crate::{Parser, ParserError, TokenValue, parser::parse_expression};

mod block;
pub use block::parse_block;
mod break_;
use break_::parse_break;
mod continue_;
use continue_::parse_continue;
mod else_;
use else_::parse_else;
mod if_;
use if_::parse_if;
mod let_;
use let_::parse_let;
mod return_;
use return_::parse_return;
mod while_;
use while_::parse_while;

pub fn parse_statement(parser: &mut Parser) -> Result<Option<Statement>, ParserError> {
	let Some(token) = parser.tokens.peek() else {
		return Ok(None);
	};

	Ok(Some(match token.value {
		TokenValue::EoF => return Ok(None),

		// statements
		TokenValue::Let => Statement::Let(parse_let(parser)?),
		TokenValue::If => Statement::If(parse_if(parser)?),
		TokenValue::While => Statement::While(parse_while(parser)?),
		TokenValue::Continue => Statement::Continue(parse_continue(parser)?),
		TokenValue::Break => Statement::Break(parse_break(parser)?),
		TokenValue::Return => Statement::Return(parse_return(parser)?),

		// expression
		_ => {
			let expr = parse_expression(parser)?;

			let Some(peek) = parser.tokens.peek() else {
				return Err(ParserError::unexpected_eof(vec![
					TokenValue::Semicolon.get_type(),
				]));
			};
			let val;
			if peek.value == TokenValue::Equal {
				parser.tokens.expect(TokenValue::Equal)?;
				if let Expression::Variable(id) = expr {
					if !id.val.scope.is_empty() {
						return Err(ParserError::scoped_identifier_assign(
							id.val.to_string(),
							id.span,
						));
					}

					let value = parse_expression(parser)?;

					val = Statement::Assignment(Node {
						span: Span {
							start: id.span.start,
							end: value.get_span().end,
						},
						val: Assignment {
							id: id.val.id,
							value,
						},
					})
				} else {
					val = Statement::Expression(expr);
				}
			} else if peek.value == TokenValue::ArrowLeft {
				parser.tokens.expect(TokenValue::ArrowLeft)?;
				let value = parse_expression(parser)?;

				val = Statement::Send(Node {
					span: Span {
						start: expr.get_span().start,
						end: value.get_span().end,
					},
					val: Send { ch: expr, value },
				})
			} else {
				val = Statement::Expression(expr);
			}

			parser.tokens.expect(TokenValue::Semicolon)?;
			val
		}
	}))
}
