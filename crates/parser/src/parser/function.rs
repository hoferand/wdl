use ast::{function::Parameter, Function, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::{identifier::parse_identifier, statement::parse_block};

pub(crate) fn parse_function(
	parser: &mut Parser,
) -> Result<Node<Span, Function<Span>>, ParserError> {
	let start = parser
		.tokens
		.expect(TokenValue::ParenOpen)?
		.span
		.start
		.clone();

	// parse parameter
	let mut parameter = Vec::new();
	while let Some(token) = parser.tokens.peek() {
		if token.value == TokenValue::ParenClose {
			break;
		}
		let id = parse_identifier(parser)?;
		parameter.push(Node {
			src: id.src.clone(),
			val: Parameter { id },
		});
		parser.tokens.want(TokenValue::Comma);
	}

	parser.tokens.expect(TokenValue::ParenClose)?;

	// parse body
	parser.state.in_function += 1;
	let body = parse_block(parser)?;
	parser.state.in_function -= 1;

	Ok(Node {
		src: Span {
			start,
			end: body.src.end.clone(),
		},
		val: Function { parameter, body },
	})
}
