use ast::{Node, Sleep, Span};

use crate::{parser::expression::parse_expression, Parser, ParserError, TokenValue};

pub(crate) fn parse_sleep(parser: &mut Parser) -> Result<Node<Sleep>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Sleep)?.span.start.clone();

	let time = parse_expression(parser)?;

	let end = parser
		.tokens
		.expect(TokenValue::Semicolon)?
		.span
		.end
		.clone();

	Ok(Node {
		span: Span { start, end },
		val: Sleep { time },
	})
}
