use ast::{BinaryOperator, Location, Node, Span, UnaryOperator};

use crate::{ParserError, Token, TokenValue};

pub(crate) struct TokenStream<'t> {
	tokens: &'t [Token],
	pointer: usize,
}

impl<'t> TokenStream<'t> {
	pub fn new(tokens: &'t [Token]) -> Self {
		Self { tokens, pointer: 0 }
	}

	pub fn peek(&mut self) -> Option<&Token> {
		self.tokens.get(self.pointer)
	}

	pub fn next(&mut self) -> Option<&Token> {
		let token = self.tokens.get(self.pointer);
		self.pointer += 1;

		token
	}

	pub fn last(&mut self) -> Option<&Token> {
		if self.pointer < 1 {
			return None;
		}
		self.tokens.get(self.pointer - 1)
	}

	pub fn expect(&mut self, expect: TokenValue) -> Result<&Token, ParserError> {
		let last_span = self.last().map(|t| t.span);
		if let Some(token) = self.next() {
			if token.value != expect {
				if expect == TokenValue::Semicolon {
					let span;
					if let Some(last_span) = last_span {
						span = Span {
							start: last_span.end,
							end: Location {
								line: last_span.end.line,
								column: last_span.end.column + 1,
							},
						};
					} else {
						span = Span {
							start: token.span.start,
							end: Location {
								line: token.span.start.line,
								column: token.span.start.column + 1,
							},
						}
					}

					Err(ParserError::ExpectedSemicolon { span })
				} else {
					Err(ParserError::UnexpectedToken {
						src: token.src.clone(),
						span: token.span,
						expected: vec![expect.type_str()],
					})
				}
			} else {
				Ok(token)
			}
		} else {
			Err(ParserError::UnexpectedEoF)
		}
	}

	pub fn want(&mut self, want: TokenValue) -> Option<&Token> {
		let token = self.tokens.get(self.pointer)?;
		if token.value == want {
			self.pointer += 1;
			return Some(token);
		}

		None
	}

	pub fn next_comp_op(&mut self) -> Option<Node<BinaryOperator>> {
		let token = self.tokens.get(self.pointer)?;

		let op = match token.value {
			TokenValue::EqualEqual => BinaryOperator::Equal,
			TokenValue::BangEqual => BinaryOperator::NotEqual,
			TokenValue::Less => BinaryOperator::Less,
			TokenValue::LessEqual => BinaryOperator::LessEqual,
			TokenValue::Greater => BinaryOperator::Greater,
			TokenValue::GreaterEqual => BinaryOperator::GreaterEqual,

			_ => return None,
		};

		self.pointer += 1;

		Some(Node {
			span: token.span,
			val: op,
		})
	}

	pub fn next_add_op(&mut self) -> Option<Node<BinaryOperator>> {
		let token = self.tokens.get(self.pointer)?;

		let op = match token.value {
			TokenValue::Plus => BinaryOperator::Add,
			TokenValue::Minus => BinaryOperator::Subtract,

			_ => return None,
		};

		self.pointer += 1;

		Some(Node {
			span: token.span,
			val: op,
		})
	}

	pub fn next_mul_op(&mut self) -> Option<Node<BinaryOperator>> {
		let token = self.tokens.get(self.pointer)?;

		let op = match token.value {
			TokenValue::Asterisk => BinaryOperator::Multiply,
			TokenValue::Slash => BinaryOperator::Divide,
			TokenValue::Percent => BinaryOperator::Modulo,

			_ => return None,
		};

		self.pointer += 1;

		Some(Node {
			span: token.span,
			val: op,
		})
	}

	pub fn next_unary_op(&mut self) -> Option<Node<UnaryOperator>> {
		let token = self.tokens.get(self.pointer)?;

		let op = match token.value {
			TokenValue::Minus => UnaryOperator::Negate,
			TokenValue::Bang => UnaryOperator::Flip,
			TokenValue::ArrowLeft => UnaryOperator::Receive,

			_ => return None,
		};

		self.pointer += 1;

		Some(Node {
			span: token.span,
			val: op,
		})
	}

	pub fn next_null_op(&mut self) -> Option<Node<BinaryOperator>> {
		let token = self.tokens.get(self.pointer)?;

		let op = match token.value {
			TokenValue::QuestionQuestion => BinaryOperator::NullCoalescing,

			_ => return None,
		};

		self.pointer += 1;

		Some(Node {
			span: token.span,
			val: op,
		})
	}
}
