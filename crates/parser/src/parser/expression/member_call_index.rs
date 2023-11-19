use ast::{Expression, FunctionCall, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::{parse_atomic, parse_expression};

pub(crate) fn parse_member_call_index(parser: &mut Parser) -> Result<Expression, ParserError> {
	let expr = parse_atomic(parser)?;

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

		return Ok(Expression::FunctionCall(Node {
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
		}));
	} else if parser.tokens.want(TokenValue::BracketOpen).is_some() {
		// parse index
		todo!()
	} else if parser.tokens.want(TokenValue::Point).is_some() {
		// parse member
		todo!()
	}

	return Ok(expr);
}
