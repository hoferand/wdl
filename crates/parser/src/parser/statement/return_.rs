use ast::{Node, Return, Span};

use crate::{parser::expression::parse_expression, Parser, ParserError, TokenValue};

pub(crate) fn parse_return(parser: &mut Parser) -> Result<Node<Return>, ParserError> {
	let token = parser.tokens.expect(TokenValue::Return)?;

	if parser.state.in_par > 0 || parser.state.in_function < 1 {
		return Err(ParserError::UnexpectedToken {
			src: token.src.clone(),
			span: token.span.clone(),
		});
	}

	let start = token.span.start.clone();

	let value = parse_expression(parser)?;

	let end = parser
		.tokens
		.expect(TokenValue::Semicolon)?
		.span
		.end
		.clone();

	Ok(Node {
		span: Span { start, end },
		val: Return { value },
	})
}
