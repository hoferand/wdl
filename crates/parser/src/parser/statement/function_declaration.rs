use ast::{Function, Node, Span};

use crate::{
	parser::{function::parse_function, identifier::parse_identifier},
	Parser, ParserError, TokenValue,
};

pub(crate) fn parse_function_declaration(
	parser: &mut Parser,
) -> Result<Node<Function>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Function)?.span.start;

	let id = parse_identifier(parser)?;

	let function = parse_function(parser)?;

	Ok(Node {
		span: Span {
			start,
			end: function.span.end,
		},
		val: Function { id, function },
	})
}
