mod global_declaration;
pub(crate) use global_declaration::parse_global_declaration;
mod actions;
pub(crate) use actions::parse_actions;
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
mod par;
pub(crate) use par::parse_par;
mod let_;
pub(crate) use let_::parse_let;
mod function_declaration;
pub(crate) use function_declaration::parse_function_declaration;

use ast::{Assignment, Declaration, Expression, Node, Send, Span, Statement};

use crate::{Parser, ParserError, TokenValue};

use super::expression::parse_expression;

pub(crate) fn parse_declaration(
	parser: &mut Parser,
) -> Result<Option<Declaration<Span>>, ParserError> {
	let Some(token) = parser.tokens.peek() else {
		return Ok(None);
	};

	Ok(Some(match token.value {
		TokenValue::EoF => return Ok(None),

		// statements
		TokenValue::Global => Declaration::GlobalDeclaration(parse_global_declaration(parser)?),
		TokenValue::Actions => Declaration::Actions(parse_actions(parser)?),
		TokenValue::Function => {
			Declaration::FunctionDeclaration(parse_function_declaration(parser)?)
		}

		_ => {
			return Err(ParserError::UnexpectedToken {
				src: token.src.clone(),
				span: token.span.clone(),
			});
		}
	}))
}

pub(crate) fn parse_statement(parser: &mut Parser) -> Result<Option<Statement<Span>>, ParserError> {
	let Some(token) = parser.tokens.peek() else {
		return Ok(None);
	};

	Ok(Some(match token.value {
		TokenValue::EoF => return Ok(None),

		// statements
		TokenValue::Let => Statement::Let(parse_let(parser)?),
		TokenValue::Par => Statement::Par(parse_par(parser)?),
		TokenValue::If => Statement::If(parse_if(parser)?),
		TokenValue::While => Statement::While(parse_while(parser)?),
		TokenValue::Continue => Statement::Continue(parse_continue(parser)?),
		TokenValue::Break => Statement::Break(parse_break(parser)?),
		TokenValue::Return => Statement::Return(parse_return(parser)?),

		// expression
		_ => {
			let expr = parse_expression(parser)?;

			let Some(peek) = parser.tokens.peek() else {
				return Err(ParserError::UnexpectedEoF);
			};
			let val;
			if peek.value == TokenValue::Equal {
				parser.tokens.expect(TokenValue::Equal)?;
				if let Expression::Identifier(id) = expr {
					if !id.val.scope.is_empty() {
						// TODO: improve error message
						return Err(ParserError::Fatal(
							"It is not allowed to assign values to scoped identifiers!".to_owned(),
						));
					}

					let value = parse_expression(parser)?;

					val = Statement::Assignment(Node {
						src: Span {
							start: id.src.start.clone(),
							end: value.get_src().end.clone(),
						},
						val: Assignment {
							id: id.val.id,
							value: Box::new(value),
						},
					})
				} else {
					val = Statement::Expression(expr);
				}
			} else if peek.value == TokenValue::ArrowLeft {
				parser.tokens.expect(TokenValue::ArrowLeft)?;
				let value = parse_expression(parser)?;

				val = Statement::Send(Node {
					src: Span {
						start: expr.get_src().start.clone(),
						end: value.get_src().end.clone(),
					},
					val: Send {
						ch: Box::new(expr),
						value: Box::new(value),
					},
				})
			} else {
				val = Statement::Expression(expr);
			}

			parser.tokens.expect(TokenValue::Semicolon)?;
			val
		}
	}))
}
