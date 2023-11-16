use ast::{Expression, Identifier, Literal, Node};

use crate::{Parser, ParserError, TokenValue};

use super::parse_expression;

pub(crate) fn parse_atomic(parser: &mut Parser) -> Result<Expression, ParserError> {
	let Some(token) = parser.tokens.next() else {
		return Err(ParserError::UnexpectedEoF);
	};

	let expr = match &token.value {
		TokenValue::Null => Expression::Literal(Node {
			span: token.span.clone(),
			val: Literal::Null,
		}),
		TokenValue::Bool(b) => Expression::Literal(Node {
			span: token.span.clone(),
			val: Literal::Bool(*b),
		}),
		TokenValue::Number(n) => Expression::Literal(Node {
			span: token.span.clone(),
			val: Literal::Number(*n),
		}),
		TokenValue::String(s) => Expression::Literal(Node {
			span: token.span.clone(),
			val: Literal::String(s.to_owned()),
		}),
		TokenValue::ParenOpen => {
			let expr = parse_expression(parser)?;
			parser.tokens.expect(TokenValue::ParenClose)?;
			expr
		}
		TokenValue::Identifier(id) => Expression::Identifier(Node {
			span: token.span.clone(),
			val: Identifier(id.to_owned()),
		}),
		// TODO: arrays, objects
		_ => {
			return Err(ParserError::UnexpectedToken {
				src: token.src.clone(),
				span: token.span.clone(),
			})
		}
	};

	Ok(expr)
}
