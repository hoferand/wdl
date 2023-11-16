use ast::{Break, Node, Span};

use crate::{Parser, ParserError, TokenValue};

pub(crate) fn parse_break(parser: &mut Parser) -> Result<Node<Break>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Break)?.span.start.clone();

	let end = parser
		.tokens
		.expect(TokenValue::Semicolon)?
		.span
		.end
		.clone();

	Ok(Node {
		span: Span { start, end },
		val: Break,
	})
}
