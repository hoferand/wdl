use ast::{Binary, Expression, Node, Span};

use crate::{Parser, ParserError};

use super::parse_additive;

pub(crate) fn parse_comparison(parser: &mut Parser) -> Result<Expression, ParserError> {
	let mut left = parse_additive(parser)?;

	while let Some(op) = parser.tokens.next_comp_op() {
		let right = parse_additive(parser)?;

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
