use ast::{Assignment, Expression, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::parse_or;

pub(crate) fn parse_assignment(parser: &mut Parser) -> Result<Expression, ParserError> {
	let expr = parse_or(parser)?;

	let Some(peek) = parser.tokens.peek() else {
		return Ok(expr);
	};
	if peek.value != TokenValue::Equal {
		return Ok(expr);
	}

	let Expression::Identifier(id) = expr else {
		return Ok(expr);
	};

	if !id.val.scope.is_empty() {
		// TODO: make better error
		return Err(ParserError::Fatal(
			"It is not allowed to assign values to scoped identifiers!".to_owned(),
		));
	}

	parser.tokens.expect(TokenValue::Equal)?;

	let value = parse_or(parser)?;

	Ok(Expression::Assignment(Node {
		span: Span {
			start: id.span.start.clone(),
			end: value.get_span().end.clone(),
		},
		val: Assignment {
			id: id.val.id,
			value: Box::new(value),
		},
	}))
}
