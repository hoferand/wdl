use ast::{Expression, Logic, LogicOperator, Node, Span};

use crate::{parser::parse_comparison, Parser, ParserError, TokenValue};

pub fn parse_and(parser: &mut Parser) -> Result<Expression, ParserError> {
	let mut left = parse_comparison(parser)?;

	while let Some(op) = parser.tokens.want(TokenValue::LAnd).cloned() {
		let right = parse_comparison(parser)?;

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
