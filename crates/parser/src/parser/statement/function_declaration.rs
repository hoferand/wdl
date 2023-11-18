use ast::{FunctionDeclaration, Node};

use crate::{Parser, ParserError, TokenValue};

pub(crate) fn parse_function_declaration(
	parser: &mut Parser,
) -> Result<Node<FunctionDeclaration>, ParserError> {
	let _start = parser
		.tokens
		.expect(TokenValue::Function)?
		.span
		.start
		.clone();
	parser.state.in_function += 1;
	// parse
	parser.state.in_function -= 1;
	todo!()
}
