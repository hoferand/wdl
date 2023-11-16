pub mod lexer_error;
pub use lexer_error::LexerError;

use std::{iter::Peekable, str::Chars};

use ast::{Location, Span};

use crate::{Token, TokenValue};

pub(crate) struct Lexer<'c> {
	chars: Peekable<Chars<'c>>,
	line: usize,
	column: usize,
	new_line: bool,
}

impl<'c> Lexer<'c> {
	pub fn new(src_code: &'c str) -> Self {
		Self {
			chars: src_code.chars().peekable(),
			line: 0,
			column: 0,
			new_line: false,
		}
	}

	pub fn get_tokens(mut self) -> Result<Vec<Token>, Vec<LexerError>> {
		let mut tokens = Vec::new();
		let mut errors = Vec::new();

		while let Some(curr_char) = self.get_char() {
			let start_line = self.line;
			let start_column = self.column - 1;

			// create token
			let value;
			let mut src = curr_char.to_string();
			match curr_char {
				// ignore whitespaces and newlines
				' ' | '\t' | '\n' => continue,

				// single character
				'+' => value = TokenValue::Plus,
				'*' => value = TokenValue::Star,
				'/' => value = TokenValue::Slash,
				'%' => value = TokenValue::Percent,
				'#' => value = TokenValue::Hash,
				'@' => value = TokenValue::At,
				'|' => value = TokenValue::Pipe,
				'.' => value = TokenValue::Point,
				',' => value = TokenValue::Comma,
				';' => value = TokenValue::Semicolon,

				'(' => value = TokenValue::ParenOpen,
				')' => value = TokenValue::ParenClose,
				'[' => value = TokenValue::BracketOpen,
				']' => value = TokenValue::BracketClose,
				'{' => value = TokenValue::CurlyOpen,
				'}' => value = TokenValue::CurlyClose,

				// two character
				'-' => match self.chars.peek() {
					Some('>') => {
						value = TokenValue::Arrow;
						src.push(self.get_char().unwrap());
					}
					_ => value = TokenValue::Minus,
				},
				':' => match self.chars.peek() {
					Some(':') => {
						value = TokenValue::ColonColon;
						src.push(self.get_char().unwrap());
					}
					_ => value = TokenValue::Colon,
				},
				'?' => match self.chars.peek() {
					Some('?') => {
						value = TokenValue::QuestionQuestion;
						src.push(self.get_char().unwrap());
					}
					_ => value = TokenValue::Question,
				},
				'=' => match self.chars.peek() {
					Some('=') => {
						value = TokenValue::EqualEqual;
						src.push(self.get_char().unwrap());
					}
					_ => value = TokenValue::Equal,
				},
				'!' => match self.chars.peek() {
					Some('=') => {
						value = TokenValue::BangEqual;
						src.push(self.get_char().unwrap());
					}
					_ => value = TokenValue::Bang,
				},
				'<' => match self.chars.peek() {
					Some('=') => {
						value = TokenValue::LessEqual;
						src.push(self.get_char().unwrap());
					}
					_ => value = TokenValue::Less,
				},
				'>' => match self.chars.peek() {
					Some('=') => {
						value = TokenValue::GreaterEqual;
						src.push(self.get_char().unwrap());
					}
					_ => value = TokenValue::Greater,
				},

				// string
				'"' => {
					let mut string = String::new();
					let mut terminated = false;
					while let Some(next_char) = self.get_char() {
						src.push(next_char);
						if next_char == '"' {
							terminated = true;
							break;
						}
						string.push(next_char);
					}
					if !terminated {
						errors.push(LexerError::UnexpectedEndOfString {
							src,
							span: Span {
								start: Location {
									line: start_line,
									column: start_column,
								},
								end: Location {
									line: self.line,
									column: self.column,
								},
							},
						});
						continue;
					}

					value = TokenValue::String(string);
				}

				// number
				c if c.is_ascii_digit() => {
					let mut point = false;
					while let Some(next_char) = self.chars.peek() {
						if next_char.is_ascii_digit() {
							src.push(self.get_char().unwrap());
						} else if !point && *next_char == '.' {
							point = true;
							src.push(self.get_char().unwrap());
						} else {
							break;
						}
					}

					if let Ok(number) = src.parse() {
						value = TokenValue::Number(number);
					} else {
						errors.push(LexerError::InvalidNumber {
							src,
							span: Span {
								start: Location {
									line: start_line,
									column: start_column,
								},
								end: Location {
									line: self.line,
									column: self.column,
								},
							},
						});
						continue;
					}
				}

				// identifier / keyword
				c if c.is_ascii_alphabetic() || c == '_' => {
					while let Some(next_char) = self.chars.peek() {
						if next_char.is_ascii_alphanumeric() || *next_char == '_' {
							src.push(self.get_char().unwrap());
						} else {
							break;
						}
					}
					value = Lexer::get_keyword(&src);
				}

				// invalid character
				_ => {
					errors.push(LexerError::InvalidCharacter {
						char: curr_char,
						loc: Location {
							line: start_line,
							column: start_column,
						},
					});
					continue;
				}
			}

			// add token
			tokens.push(Token {
				value,
				span: Span {
					start: Location {
						line: start_line,
						column: start_column,
					},
					end: Location {
						line: self.line,
						column: self.column,
					},
				},
				src,
			});
		}

		if !errors.is_empty() {
			Err(errors)
		} else {
			tokens.push(Token {
				value: TokenValue::EoF,
				span: Span {
					start: Location {
						line: self.line,
						column: self.column,
					},
					end: Location {
						line: self.line,
						column: self.column + 1,
					},
				},
				src: "EoF".to_owned(),
			});

			Ok(tokens)
		}
	}

	fn get_char(&mut self) -> Option<char> {
		let curr_char = self.chars.next();
		if self.new_line {
			self.new_line = false;
			self.line += 1;
			self.column = 0;
		}
		if let Some(char) = curr_char {
			self.column += 1;
			if char == '\n' {
				self.new_line = true;
			}
		}

		curr_char
	}

	fn get_keyword(id: &str) -> TokenValue {
		match id {
			"null" => TokenValue::Null,
			"true" => TokenValue::Bool(true),
			"false" => TokenValue::Bool(false),
			"use" => TokenValue::Use,
			"mod" => TokenValue::Mod,
			"import" => TokenValue::Import,
			"global" => TokenValue::Global,
			"order" => TokenValue::Order,
			"fn" => TokenValue::Fn,
			"let" => TokenValue::Let,
			"par" => TokenValue::Par,
			"sleep" => TokenValue::Sleep,
			"print" => TokenValue::Print,
			"and" => TokenValue::LAnd,
			"or" => TokenValue::LOr,
			"if" => TokenValue::If,
			"else" => TokenValue::Else,
			"while" => TokenValue::While,
			"for" => TokenValue::For,
			"continue" => TokenValue::Continue,
			"break" => TokenValue::Break,
			"return" => TokenValue::Return,
			"bool" => TokenValue::TBool,
			"number" => TokenValue::TNumber,
			"string" => TokenValue::TString,
			"any" => TokenValue::TAny,
			_ => TokenValue::Identifier(id.to_owned()),
		}
	}
}
