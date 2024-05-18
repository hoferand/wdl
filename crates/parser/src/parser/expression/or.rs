use ast::{Expression, Logic, LogicOperator, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::parse_and;

pub fn parse_or(parser: &mut Parser) -> Result<Expression, ParserError> {
	let mut left = parse_and(parser)?;

	while let Some(op) = parser.tokens.want(TokenValue::LOr).cloned() {
		let right = parse_and(parser)?;

		left = Expression::Logic(Node {
			span: Span {
				start: left.get_span().start,
				end: right.get_span().end,
			},
			val: Logic {
				left: Box::new(left),
				op: Node {
					span: op.span,
					val: LogicOperator::Or,
				},
				right: Box::new(right),
			},
		})
	}

	Ok(left)
}
