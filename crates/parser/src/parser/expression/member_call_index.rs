use ast::{Expression, FunctionCall, Member, Node, Offset, Span};

use crate::{parser::identifier::parse_identifier, Parser, ParserError, TokenValue};

use super::{parse_atomic, parse_expression};

pub(crate) fn parse_member_call_index(
	parser: &mut Parser,
) -> Result<Expression<Span>, ParserError> {
	let mut expr = parse_atomic(parser)?;

	loop {
		if let Some(paren) = parser.tokens.want(TokenValue::ParenOpen) {
			// parse function call
			let start = paren.span.start.clone();

			let mut parameter = Vec::new();
			while let Some(token) = parser.tokens.peek() {
				if token.value == TokenValue::ParenClose {
					break;
				}
				parameter.push(parse_expression(parser)?);
				parser.tokens.want(TokenValue::Comma);
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
					parameter: Node {
						src: Span { start, end },
						val: parameter,
					},
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
