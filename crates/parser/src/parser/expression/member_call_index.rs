use ast::{Argument, Expression, FunctionCall, Member, Node, Offset, Span};

use crate::{parser::identifier::parse_identifier, Parser, ParserError, TokenValue};

use super::{parse_atomic, parse_expression};

pub(crate) fn parse_member_call_index(
	parser: &mut Parser,
) -> Result<Expression<Span>, ParserError> {
	let mut expr = parse_atomic(parser)?;

	loop {
		if parser.tokens.want(TokenValue::ParenOpen).is_some() {
			// parse function call
			let mut args = Vec::new();
			while let Some(token) = parser.tokens.peek() {
				if token.value == TokenValue::ParenClose {
					break;
				}
				let id = parse_expression(parser)?;

				if let Some(token) = parser.tokens.want(TokenValue::Colon).cloned() {
					let val = parse_expression(parser)?;
					if let Expression::Identifier(id) = id {
						if !id.val.scope.is_empty() {
							return Err(ParserError::Positional {
								msg: format!(
									"Cannot use scoped identifier `{}` as named argument",
									id.val
								),
								span: id.src,
							});
						}
						args.push(Node {
							src: Span {
								start: id.src.start,
								end: val.get_src().end.clone(),
							},
							val: Argument {
								id: Some(Node {
									src: id.val.id.src,
									val: id.val.id.val,
								}),
								val,
							},
						});
					} else {
						return Err(ParserError::UnexpectedToken {
							src: token.src,
							span: token.span,
						});
					}
				} else {
					args.push(Node {
						src: id.get_src().clone(),
						val: Argument { id: None, val: id },
					});
				}

				if parser.tokens.want(TokenValue::Comma).is_none() {
					break;
				}
			}

			let end = parser
				.tokens
				.expect(TokenValue::ParenClose)?
				.span
				.end
				.clone();

			expr = Expression::FunctionCall(Node {
				src: Span {
					start: expr.get_src().start.clone(),
					end: end.clone(),
				},
				val: FunctionCall {
					function: Box::new(expr),
					args,
				},
			});
		} else if let Some(bracket) = parser.tokens.want(TokenValue::BracketOpen) {
			// parse index
			let start = bracket.span.start.clone();

			let offset = parse_expression(parser)?;

			let end = parser
				.tokens
				.expect(TokenValue::BracketClose)?
				.span
				.end
				.clone();

			expr = Expression::Offset(Node {
				src: Span { start, end },
				val: Offset {
					value: Box::new(expr),
					offset: Box::new(offset),
				},
			});
		} else if let Some(point) = parser.tokens.want(TokenValue::Point) {
			// parse member
			let start = point.span.start.clone();

			let id = parse_identifier(parser)?;

			expr = Expression::Member(Node {
				src: Span {
					start,
					end: id.src.end.clone(),
				},
				val: Member {
					object: Box::new(expr),
					member: id,
				},
			})
		} else {
			break;
		}
	}

	Ok(expr)
}
