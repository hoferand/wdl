use ast::{Expression, Logical, LogicalOperator, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::parse_and;

pub(crate) fn parse_or(parser: &mut Parser) -> Result<Expression<Span>, ParserError> {
	let mut left = parse_and(parser)?;

	while let Some(op) = parser.tokens.want(TokenValue::LOr).cloned() {
		let right = parse_and(parser)?;

		left = Expression::Logical(Node {
			src: Span {
				start: left.get_src().start.clone(),
				end: right.get_src().end.clone(),
			},
			val: Logical {
				left: Box::new(left),
				op: Node {
					src: op.span,
					val: LogicalOperator::Or,
				},
				right: Box::new(right),
			},
		})
	}

	Ok(left)
}
