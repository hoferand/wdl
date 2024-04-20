use ast::{Identifier, Let, Node, Span};

use crate::{parser::parse_expression, Parser, ParserError, TokenValue};

pub(crate) fn parse_let(parser: &mut Parser) -> Result<Node<Span, Let<Span>>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Let)?.span.start;

	let Some(id_token) = parser.tokens.next().cloned() else {
		return Err(ParserError::UnexpectedEoF);
	};
	let TokenValue::Identifier(id) = &id_token.value else {
		return Err(ParserError::UnexpectedToken {
			src: id_token.src.clone(),
			span: id_token.span,
		});
	};

	parser.tokens.expect(TokenValue::Equal)?;

	// TODO: make it optional
	let value = parse_expression(parser)?;

	let end = parser
		.tokens
		.expect(TokenValue::Semicolon)?
		.span
		.end;

	let id_node = Node {
		src: Span {
			start,
			end,
		},
		val: Identifier(id.to_owned()),
	};

	Ok(Node {
		src: Span { start, end },
		val: Let { id: id_node, value },
	})
}
