use ast::{function::FormalParameter, FunctionBody, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::{identifier::parse_identifier, statement::parse_block};

pub(crate) fn parse_function(parser: &mut Parser) -> Result<Node<FunctionBody>, ParserError> {
	let start = parser.tokens.expect(TokenValue::ParenOpen)?.span.start;

	// parse parameter
	let mut parameter = Vec::new();
	while let Some(token) = parser.tokens.peek() {
		if token.value == TokenValue::ParenClose {
			break;
		}
		let id = parse_identifier(parser)?;
		parameter.push(Node {
			span: id.span,
			val: FormalParameter { id },
		});
		parser.tokens.want(TokenValue::Comma);
	}

	parser.tokens.expect(TokenValue::ParenClose)?;

	// parse body
	parser.state.in_function += 1;
	let body = parse_block(parser)?;
	parser.state.in_function -= 1;

	Ok(Node {
		span: Span {
			start,
			end: body.span.end,
		},
		val: FunctionBody { parameter, body },
	})
}
