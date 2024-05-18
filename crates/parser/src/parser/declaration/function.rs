use ast::{FormalParameter, Function, FunctionBody, Node, Span};

use crate::{
	parser::{identifier::parse_identifier, parse_block},
	Parser, ParserError, TokenValue,
};

pub fn parse_function(parser: &mut Parser) -> Result<Node<Function>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Function)?.span.start;

	let id = parse_identifier(parser)?;

	parser.tokens.expect(TokenValue::ParenOpen)?;

	// parse parameters
	let mut parameters: Vec<Node<FormalParameter>> = Vec::new();
	while let Some(token) = parser.tokens.peek() {
		if token.value == TokenValue::ParenClose {
			break;
		}
		let id = parse_identifier(parser)?;
		if parameters.iter().any(|p| p.val.id.val.id == id.val.id) {
			return Err(ParserError::duplicate_parameter(id.val.id, id.span));
		}
		parameters.push(Node {
			span: id.span,
			val: FormalParameter { id },
		});

		if parser.tokens.want(TokenValue::Comma).is_none() {
			break;
		}
	}

	parser.tokens.expect(TokenValue::ParenClose)?;

	// parse body
	parser.state.in_function += 1;
	let body = parse_block(parser)?;
	parser.state.in_function -= 1;

	let function = Node {
		span: Span {
			start,
			end: body.span.end,
		},
		val: FunctionBody { parameters, body },
	};

	Ok(Node {
		span: Span {
			start,
			end: function.span.end,
		},
		val: Function { id, function },
	})
}
