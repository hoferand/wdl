use std::collections::HashMap;

use ast::{Array, Expression, Group, Identifier, Literal, Node, Object, Span, Variable};

use crate::{Parser, ParserError, TokenValue, parser::parse_expression};

pub fn parse_atomic(parser: &mut Parser) -> Result<Expression, ParserError> {
	let Some(token) = parser.tokens.next() else {
		return Err(ParserError::unexpected_eof(vec![
			TokenValue::Null.get_type(),
			TokenValue::Bool(false).get_type(),
			TokenValue::Number(0.0).get_type(),
			TokenValue::String(String::new()).get_type(),
			TokenValue::Identifier(String::new()).get_type(),
			TokenValue::ParenOpen.get_type(),
			TokenValue::BracketOpen.get_type(),
			TokenValue::CurlyOpen.get_type(),
		]));
	};

	let expr = match &token.value {
		TokenValue::Null => Expression::Literal(Node {
			span: token.span,
			val: Literal::Null,
		}),
		TokenValue::Bool(b) => Expression::Literal(Node {
			span: token.span,
			val: Literal::Bool(*b),
		}),
		TokenValue::Number(n) => Expression::Literal(Node {
			span: token.span,
			val: Literal::Number(*n),
		}),
		TokenValue::String(s) => Expression::Literal(Node {
			span: token.span,
			val: Literal::String(s.to_owned()),
		}),
		TokenValue::Identifier(id) => {
			let mut scope = Vec::new();
			scope.push(Node {
				span: token.span,
				val: Identifier { id: id.to_owned() },
			});

			while let Some(token) = parser.tokens.peek() {
				if token.value != TokenValue::ColonColon {
					break;
				}
				parser.tokens.expect(TokenValue::ColonColon)?;

				let Some(id) = parser.tokens.next() else {
					return Err(ParserError::unexpected_eof(vec![
						TokenValue::Identifier(String::new()).get_type(),
					]));
				};

				if let TokenValue::Identifier(id_str) = &id.value {
					scope.push(Node {
						span: id.span,
						val: Identifier {
							id: id_str.to_owned(),
						},
					});
				}
			}

			let id = scope.pop().unwrap(); // can not fail

			let start;
			if let Some(sc) = scope.first() {
				start = sc.span.start;
			} else {
				start = id.span.start;
			}
			let end = id.span.end;

			Expression::Variable(Node {
				span: Span { start, end },
				val: Variable { id, scope },
			})
		}
		TokenValue::ParenOpen => {
			let start = token.span.start;
			let expr = parse_expression(parser)?;
			let end = parser.tokens.expect(TokenValue::ParenClose)?.span.end;

			Expression::Group(Node {
				span: Span { start, end },
				val: Group {
					expression: Box::new(expr),
				},
			})
		}
		TokenValue::BracketOpen => {
			let start = token.span.start;

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

			let end = parser.tokens.expect(TokenValue::BracketClose)?.span.end;

			Expression::Array(Node {
				span: Span { start, end },
				val: Array { values },
			})
		}
		TokenValue::CurlyOpen => {
			let start = token.span.start;

			let mut values = HashMap::new();
			while let Some(token) = parser.tokens.peek() {
				if token.value == TokenValue::CurlyClose {
					break;
				}

				let Some(key_token) = parser.tokens.next() else {
					return Err(ParserError::unexpected_eof(vec![
						TokenValue::Identifier(String::new()).get_type(),
						TokenValue::String(String::new()).get_type(),
					]));
				};

				let key = match &key_token.value {
					TokenValue::Identifier(id) => id,
					TokenValue::String(s) => s,
					_ => {
						return Err(ParserError::unexpected_token(
							key_token.src.clone(),
							vec![
								TokenValue::Identifier(String::new()).get_type(),
								TokenValue::String(String::new()).get_type(),
							],
							key_token.span,
						));
					}
				}
				.to_owned();

				parser.tokens.expect(TokenValue::Colon)?;

				values.insert(key, parse_expression(parser)?);

				if parser.tokens.want(TokenValue::Comma).is_none() {
					break;
				}
			}

			let end = parser.tokens.expect(TokenValue::CurlyClose)?.span.end;

			Expression::Object(Node {
				span: Span { start, end },
				val: Object { values },
			})
		}
		_ => {
			return Err(ParserError::unexpected_token(
				token.src.clone(),
				vec![
					TokenValue::Null.get_type(),
					TokenValue::Bool(false).get_type(),
					TokenValue::Number(0.0).get_type(),
					TokenValue::String(String::new()).get_type(),
					TokenValue::Identifier(String::new()).get_type(),
					TokenValue::ParenOpen.get_type(),
					TokenValue::BracketOpen.get_type(),
					TokenValue::CurlyOpen.get_type(),
				],
				token.span,
			));
		}
	};

	Ok(expr)
}
