use ast::{Binary, Expression, Node, Span};

use crate::{Parser, ParserError};

use super::parse_multiplicative;

pub(crate) fn parse_additive(parser: &mut Parser) -> Result<Expression, ParserError> {
	let mut left = parse_multiplicative(parser)?;

	while let Some(op) = parser.tokens.next_add_op() {
		let right = parse_multiplicative(parser)?;

		left = Expression::Binary(Node {
			span: Span {
				start: left.get_span().start.clone(),
				end: right.get_span().end.clone(),
			},
			val: Binary {
				left: Box::new(left),
				op,
				right: Box::new(right),
			},
		})
	}

	Ok(left)
}