use ast::{FunctionDeclaration, Node, Span};

use crate::{
	parser::{function::parse_function, identifier::parse_identifier},
	Parser, ParserError, TokenValue,
};

pub(crate) fn parse_function_declaration(
	parser: &mut Parser,
) -> Result<Node<FunctionDeclaration>, ParserError> {
	let start = parser
		.tokens
		.expect(TokenValue::Function)?
		.span
		.start
		.clone();

	let id = parse_identifier(parser)?;

	let function = parse_function(parser)?;

	Ok(Node {
		span: Span {
			start,
			end: function.span.end.clone(),
		},
		val: FunctionDeclaration { id, function },
	})
}
