use ast::{GlobalDeclaration, Identifier, Node, Span};

use crate::{
	parser::{expression::parse_expression, type_::parse_type},
	Parser, ParserError, TokenValue,
};

pub(crate) fn parse_global_declaration(
	parser: &mut Parser,
) -> Result<Node<GlobalDeclaration>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Global)?.span.start.clone();

	let Some(id_token) = parser.tokens.next().cloned() else {
		return Err(ParserError::UnexpectedEoF);
	};
	let TokenValue::Identifier(id) = &id_token.value else {
		return Err(ParserError::UnexpectedToken {
			src: id_token.src.clone(),
			span: id_token.span.clone(),
		});
	};

	parser.tokens.expect(TokenValue::Colon)?;

	let type_ = parse_type(parser)?;

	let id_node = Node {
		span: Span {
			start: id_token.span.start.clone(),
			end: type_.span.end.clone(),
		},
		val: Identifier(id.to_owned()),
	};

	parser.tokens.expect(TokenValue::Equal)?;

	// TODO: make it optional
	let value = parse_expression(parser)?;

	// TODO: parse global description

	let end = parser
		.tokens
		.expect(TokenValue::Semicolon)?
		.span
		.end
		.clone();

	Ok(Node {
		span: Span { start, end },
		val: GlobalDeclaration {
			id: id_node,
			type_,
			value: Some(value),
			description: None,
		},
	})
}
