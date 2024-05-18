use ast::{Argument, Call, Expression, Member, Node, Offset, Span};

use crate::{parser::identifier::parse_identifier, Parser, ParserError, TokenValue};

use super::{parse_atomic, parse_expression};

pub(crate) fn parse_member_call_index(parser: &mut Parser) -> Result<Expression, ParserError> {
	let mut expr = parse_atomic(parser)?;

	let mut named = false;
	loop {
		if parser.tokens.want(TokenValue::ParenOpen).is_some() {
			// parse function call
			let mut args = Vec::new();
			let mut names = Vec::new();
			while let Some(token) = parser.tokens.peek() {
				if token.value == TokenValue::ParenClose {
					break;
				}
				let id = parse_expression(parser)?;

				if let Some(token) = parser.tokens.want(TokenValue::Colon).cloned() {
					named = true;
					let val = parse_expression(parser)?;
					if let Expression::Variable(id) = id {
						if !id.val.scope.is_empty() {
							return Err(ParserError::scoped_argument(id.val.to_string(), id.span));
						}

						if names.contains(&id.val.id.val) {
							return Err(ParserError::duplicate_argument(
								id.val.id.val.id,
								Span {
									start: id.span.start,
									end: val.get_span().end,
								},
							));
						}
						names.push(id.val.id.val.clone());

						args.push(Node {
							span: Span {
								start: id.span.start,
								end: val.get_span().end,
							},
							val: Argument {
								id: Some(Node {
									span: id.val.id.span,
									val: id.val.id.val,
								}),
								val,
							},
						});
					} else {
						return Err(ParserError::unexpected_token(
							token.src,
							vec![TokenValue::Comma.get_type()],
							token.span,
						));
					}
				} else if !named {
					args.push(Node {
						span: *id.get_span(),
						val: Argument { id: None, val: id },
					});
				} else {
					return Err(ParserError::positional_after_named(*id.get_span()));
				}

				if parser.tokens.want(TokenValue::Comma).is_none() {
					break;
				}
			}

			let end = parser.tokens.expect(TokenValue::ParenClose)?.span.end;

			expr = Expression::Call(Node {
				span: Span {
					start: expr.get_span().start,
					end,
				},
				val: Call {
					function: Box::new(expr),
					args,
				},
			});
		} else if let Some(bracket) = parser.tokens.want(TokenValue::BracketOpen) {
			// parse index
			let start = bracket.span.start;

			let offset = parse_expression(parser)?;

			let end = parser.tokens.expect(TokenValue::BracketClose)?.span.end;

			expr = Expression::Offset(Node {
				span: Span { start, end },
				val: Offset {
					value: Box::new(expr),
					offset: Box::new(offset),
				},
			});
		} else if let Some(point) = parser.tokens.want(TokenValue::Point) {
			// parse member
			let start = point.span.start;

			let id = parse_identifier(parser)?;

			expr = Expression::Member(Node {
				span: Span {
					start,
					end: id.span.end,
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
