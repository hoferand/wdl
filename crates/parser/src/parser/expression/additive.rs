use ast::{Binary, Expression, Node, Span};

use crate::{Parser, ParserError};

use super::parse_multiplicative;

pub(crate) fn parse_additive(parser: &mut Parser) -> Result<Expression<Span>, ParserError> {
	let mut left = parse_multiplicative(parser)?;

	while let Some(op) = parser.tokens.next_add_op() {
		let right = parse_multiplicative(parser)?;

		left = Expression::Binary(Node {
			src: Span {
				start: left.get_src().start.clone(),
				end: right.get_src().end.clone(),
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
