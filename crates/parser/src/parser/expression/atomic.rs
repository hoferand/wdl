use std::collections::HashMap;

use ast::{Array, Expression, Group, Identifier, Literal, Node, Object, Span};

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
			let start = token.span.start.clone();
			let expr = parse_expression(parser)?;
			let end = parser
				.tokens
				.expect(TokenValue::ParenClose)?
				.span
				.end
				.clone();

			Expression::Group(Node {
				span: Span { start, end },
				val: Group {
					expression: Box::new(expr),
				},
			})
		}
		TokenValue::Identifier(id) => Expression::Identifier(Node {
			span: token.span.clone(),
			val: Identifier(id.to_owned()),
		}),
		TokenValue::BracketOpen => {
			let start = token.span.start.clone();

			let mut values = Vec::new();

			while let Some(token) = parser.tokens.peek() {
				if token.value == TokenValue::BracketClose {
					break;
				}

				values.push(parse_expression(parser)?);

				if parser.tokens.want(TokenValue::Comma).is_none() {
					break;
				}
			}

			let end = parser
				.tokens
				.expect(TokenValue::BracketClose)?
				.span
				.end
				.clone();

			Expression::Array(Node {
				span: Span { start, end },
				val: Array { values },
			})
		}
		TokenValue::CurlyOpen => {
			let start = token.span.start.clone();

			let mut values = HashMap::new();
			while let Some(token) = parser.tokens.peek() {
				if token.value == TokenValue::CurlyClose {
					break;
				}

				let Some(key_token) = parser.tokens.next() else {
					return Err(ParserError::UnexpectedEoF);
				};

				let key = match &key_token.value {
					TokenValue::Identifier(id) => id,
					TokenValue::String(s) => s,
					_ => {
						return Err(ParserError::UnexpectedToken {
							src: key_token.src.clone(),
							span: key_token.span.clone(),
						});
					}
				}
				.to_owned();

				parser.tokens.expect(TokenValue::Colon)?;

				values.insert(key, parse_expression(parser)?);

				if parser.tokens.want(TokenValue::Comma).is_none() {
					break;
				}
			}

			let end = parser
				.tokens
				.expect(TokenValue::CurlyClose)?
				.span
				.end
				.clone();

			Expression::Object(Node {
				span: Span { start, end },
				val: Object { values },
			})
		}
		_ => {
			return Err(ParserError::UnexpectedToken {
				src: token.src.clone(),
				span: token.span.clone(),
			});
		}
	};

	Ok(expr)
}
