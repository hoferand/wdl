use ast::{Expression, Node, Span, Spawn};

use crate::{Parser, ParserError, TokenValue};

mod additive;
use additive::parse_additive;
mod and;
use and::parse_and;
mod atomic;
use atomic::parse_atomic;
mod comparison;
use comparison::parse_comparison;
mod member_call_index;
use member_call_index::parse_member_call_index;
mod multiplicative;
use multiplicative::parse_multiplicative;
mod null_coalescing;
use null_coalescing::parse_null_coalescing;
mod or;
use or::parse_or;
mod unary;
use unary::parse_unary;

pub fn parse_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
	let spawn_option = parser.tokens.want(TokenValue::Spawn).cloned();

	let expr = parse_or(parser)?;

	if let Some(spawn) = spawn_option {
		Ok(Expression::Spawn(Node {
			span: Span {
				start: spawn.span.start,
				end: expr.get_span().end,
			},
			val: Spawn {
				expr: Box::new(expr),
			},
		}))
	} else {
		Ok(expr)
	}
}
