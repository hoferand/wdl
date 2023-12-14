use ast::{Expression, FunctionCall, Node, Offset, Span};

use crate::{Parser, ParserError, TokenValue};

use super::{parse_atomic, parse_expression};

pub(crate) fn parse_member_call_index(parser: &mut Parser) -> Result<Expression, ParserError> {
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
				span: Span {
					start: expr.get_span().start.clone(),
					end: end.clone(),
				},
				val: FunctionCall {
					function: Box::new(expr),
					parameter: Node {
						span: Span { start, end },
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
				span: Span { start, end },
				val: Offset {
					value: Box::new(expr),
					offset: Box::new(offset),
				},
			});
		} else if parser.tokens.want(TokenValue::Point).is_some() {
			// parse member
			todo!()
		} else {
			break;
		}
	}

	Ok(expr)
}
