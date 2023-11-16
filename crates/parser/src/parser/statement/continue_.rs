use ast::{Continue, Node, Span};

use crate::{Parser, ParserError, TokenValue};

pub(crate) fn parse_continue(parser: &mut Parser) -> Result<Node<Continue>, ParserError> {
	let start = parser
		.tokens
		.expect(TokenValue::Continue)?
		.span
		.start
		.clone();

	let end = parser
		.tokens
		.expect(TokenValue::Semicolon)?
		.span
		.end
		.clone();

	Ok(Node {
		span: Span { start, end },
		val: Continue,
	})
}
