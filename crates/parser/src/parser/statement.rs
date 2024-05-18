use ast::{Assignment, Declaration, Expression, Node, Send, Span, Statement};

use crate::{Parser, ParserError, TokenValue};

use super::expression::parse_expression;

pub(crate) mod global_declaration;
pub(crate) use global_declaration::parse_global_declaration;
pub(crate) mod actions;
pub(crate) use actions::parse_actions;
pub(crate) mod block;
pub(crate) use block::parse_block;
pub(crate) mod return_;
pub(crate) use return_::parse_return;
pub(crate) mod break_;
pub(crate) use break_::parse_break;
pub(crate) mod continue_;
pub(crate) use continue_::parse_continue;
pub(crate) mod while_;
pub(crate) use while_::parse_while;
pub(crate) mod else_;
pub(crate) use else_::parse_else;
pub(crate) mod if_;
pub(crate) use if_::parse_if;
pub(crate) mod let_;
pub(crate) use let_::parse_let;
pub(crate) mod function_declaration;
pub(crate) use function_declaration::parse_function_declaration;

pub(crate) fn parse_declaration(parser: &mut Parser) -> Result<Option<Declaration>, ParserError> {
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
				span: token.span,
				expected: vec![
					TokenValue::Global.type_str(),
					TokenValue::Actions.type_str(),
					TokenValue::Function.type_str(),
				],
			});
		}
	}))
}

pub(crate) fn parse_statement(parser: &mut Parser) -> Result<Option<Statement>, ParserError> {
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
				return Err(ParserError::UnexpectedEoF);
			};
			let val;
			if peek.value == TokenValue::Equal {
				parser.tokens.expect(TokenValue::Equal)?;
				if let Expression::Variable(id) = expr {
					if !id.val.scope.is_empty() {
						// TODO: improve error message
						return Err(ParserError::Fatal(
							"It is not allowed to assign values to scoped identifiers".to_owned(),
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
