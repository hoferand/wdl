use ast::{Node, Print, Span};

use crate::{parser::expression::parse_expression, Parser, ParserError, TokenValue};

pub(crate) fn parse_print(parser: &mut Parser) -> Result<Node<Print>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Print)?.span.start.clone();

	let value = parse_expression(parser)?;

	let end = parser
		.tokens
		.expect(TokenValue::Semicolon)?
		.span
		.end
		.clone();

	Ok(Node {
		span: Span { start, end },
		val: Print { value },
	})
}
