use ast::{Expression, Node, Send, Span};

use crate::{Parser, ParserError, TokenValue};

use super::parse_or;

pub(crate) fn parse_send(parser: &mut Parser) -> Result<Expression, ParserError> {
	let ch = parse_or(parser)?;

	let Some(peek) = parser.tokens.peek() else {
		return Ok(ch);
	};
	if peek.value != TokenValue::ArrowLeft {
		return Ok(ch);
	}
	parser.tokens.expect(TokenValue::ArrowLeft)?;

	let value = parse_or(parser)?;

	Ok(Expression::Send(Node {
		span: Span {
			start: ch.get_span().start.clone(),
			end: value.get_span().end.clone(),
		},
		val: Send {
			ch: Box::new(ch),
			value: Box::new(value),
		},
	}))
}
