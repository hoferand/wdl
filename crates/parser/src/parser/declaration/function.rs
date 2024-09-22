use ast::{Function, Identifier, Node, Span};

use crate::{
	parser::{parse_block, parse_identifier},
	Parser, ParserError, TokenValue,
};

pub fn parse_function(parser: &mut Parser) -> Result<Node<Function>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Function)?.span.start;

	let id = parse_identifier(parser)?;

	parser.tokens.expect(TokenValue::ParenOpen)?;

	// parse parameters
	let mut params: Vec<Node<Identifier>> = Vec::new();
	while let Some(token) = parser.tokens.peek() {
		if token.value == TokenValue::ParenClose {
			break;
		}
		let id = parse_identifier(parser)?;
		if params.iter().any(|p| p.val.id == id.val.id) {
			return Err(ParserError::duplicate_parameter(id.val.id, id.span));
		}
		params.push(id);

		if parser.tokens.want(TokenValue::Comma).is_none() {
			break;
		}
	}

	parser.tokens.expect(TokenValue::ParenClose)?;

	// parse body
	parser.state.enter_function();
	let body = parse_block(parser)?;
	parser.state.leave_function();

	Ok(Node {
		span: Span {
			start,
			end: body.span.end,
		},
		val: Function { id, params, body },
	})
}
