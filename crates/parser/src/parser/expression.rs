use ast::{Expression, Node, Span, Spawn};

use crate::{Parser, ParserError, TokenValue};

pub mod additive;
pub use additive::parse_additive;
pub mod and;
pub use and::parse_and;
pub mod atomic;
pub use atomic::parse_atomic;
pub mod comparison;
pub use comparison::parse_comparison;
pub mod member_call_index;
pub use member_call_index::parse_member_call_index;
pub mod multiplicative;
pub use multiplicative::parse_multiplicative;
pub mod null_coalescing;
pub use null_coalescing::parse_null_coalescing;
pub mod or;
pub use or::parse_or;
pub mod unary;
pub use unary::parse_unary;

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
