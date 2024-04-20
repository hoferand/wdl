use ast::{Block, Node, Span};

use crate::{Parser, ParserError, Token, TokenValue};

use super::parse_statement;

pub(crate) fn parse_block(parser: &mut Parser) -> Result<Node<Span, Block<Span>>, ParserError> {
	let start = parser.tokens.expect(TokenValue::CurlyOpen)?.span.start;

	let mut stmts = Vec::new();
	loop {
		if let Some(Token {
			value: TokenValue::CurlyClose,
			..
		}) = parser.tokens.peek()
		{
			break;
		}

		let Some(stmt) = parse_statement(parser)? else {
			return Err(ParserError::UnexpectedEoF);
		};

		// TODO: add checks: break only in loops, etc

		stmts.push(stmt);
	}

	let end = parser.tokens.expect(TokenValue::CurlyClose)?.span.end;

	Ok(Node {
		src: Span { start, end },
		val: Block { stmts },
	})
}
