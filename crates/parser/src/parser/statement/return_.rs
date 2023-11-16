use ast::{Node, Return, Span};

use crate::{parser::expression::parse_expression, Parser, ParserError, TokenValue};

pub(crate) fn parse_return(parser: &mut Parser) -> Result<Node<Return>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Return)?.span.start.clone();

	let value = parse_expression(parser)?;

	let end = parser
		.tokens
		.expect(TokenValue::Semicolon)?
		.span
		.end
		.clone();

	Ok(Node {
		span: Span { start, end },
		val: Return { value },
	})
}
