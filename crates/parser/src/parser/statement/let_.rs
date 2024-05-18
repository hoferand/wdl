use ast::{Let, Node, Span};

use crate::{
	parser::{identifier::parse_identifier, parse_expression},
	Parser, ParserError, TokenValue,
};

pub(crate) fn parse_let(parser: &mut Parser) -> Result<Node<Let>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Let)?.span.start;

	let id_node = parse_identifier(parser)?;

	parser.tokens.expect(TokenValue::Equal)?;

	// TODO: make it optional
	let value = parse_expression(parser)?;

	let end = parser.tokens.expect(TokenValue::Semicolon)?.span.end;

	Ok(Node {
		span: Span { start, end },
		val: Let { id: id_node, value },
	})
}
