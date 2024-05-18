use ast::{Global, Identifier, Node, Span};

use crate::{parser::expression::parse_expression, Parser, ParserError, TokenValue};

pub(crate) fn parse_global_declaration(parser: &mut Parser) -> Result<Node<Global>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Global)?.span.start;

	let Some(id_token) = parser.tokens.next().cloned() else {
		return Err(ParserError::UnexpectedEoF);
	};
	let TokenValue::Identifier(id) = &id_token.value else {
		return Err(ParserError::UnexpectedToken {
			src: id_token.src.clone(),
			span: id_token.span,
			expected: vec![TokenValue::Identifier(String::new()).type_str()],
		});
	};

	let id_node = Node {
		span: id_token.span,
		val: Identifier { id: id.to_owned() },
	};

	parser.tokens.expect(TokenValue::Equal)?;

	// TODO: make it optional
	let value = parse_expression(parser)?;

	// TODO: parse global description

	let end = parser.tokens.expect(TokenValue::Semicolon)?.span.end;

	Ok(Node {
		span: Span { start, end },
		val: Global {
			id: id_node,
			value: Some(value),
		},
	})
}
