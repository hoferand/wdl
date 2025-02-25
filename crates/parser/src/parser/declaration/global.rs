use ast::{Global, Identifier, Node, Span};

use crate::{Parser, ParserError, TokenValue, parser::parse_expression};

pub fn parse_global(parser: &mut Parser) -> Result<Node<Global>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Global)?.span.start;

	let Some(id_token) = parser.tokens.next().cloned() else {
		return Err(ParserError::unexpected_eof(vec![
			TokenValue::Identifier(String::new()).get_type(),
		]));
	};
	let TokenValue::Identifier(id) = &id_token.value else {
		return Err(ParserError::unexpected_token(
			id_token.src.clone(),
			vec![TokenValue::Identifier(String::new()).get_type()],
			id_token.span,
		));
	};

	let id_node = Node {
		span: id_token.span,
		val: Identifier { id: id.to_owned() },
	};

	parser.tokens.expect(TokenValue::Equal)?;

	let value = parse_expression(parser)?;

	let end = parser.tokens.expect(TokenValue::Semicolon)?.span.end;

	Ok(Node {
		span: Span { start, end },
		val: Global { id: id_node, value },
	})
}
