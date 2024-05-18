use ast::{Block, Node, Span};

use crate::{Parser, ParserError, Token, TokenValue};

use super::parse_statement;

pub fn parse_block(parser: &mut Parser) -> Result<Node<Block>, ParserError> {
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
			return Err(ParserError::unexpected_eof(vec![
				TokenValue::CurlyClose.get_type()
			]));
		};

		stmts.push(stmt);
	}

	let end = parser.tokens.expect(TokenValue::CurlyClose)?.span.end;

	Ok(Node {
		span: Span { start, end },
		val: Block { stmts },
	})
}
