use std::{iter::Peekable, str::Chars};

use unicode_ident::{is_xid_continue, is_xid_start};

use ast::{Location, Span};

use crate::{Token, TokenValue};

pub mod lexer_error;
use lexer_error::*;

pub struct Lexer<'c> {
	chars: Peekable<Chars<'c>>,
	line: usize,
	start_line: usize,
	column: usize,
	start_column: usize,
	curr_src: String,
	new_line: bool,
}

impl<'c> Lexer<'c> {
	pub fn new(src_code: &'c str) -> Self {
		Self {
			chars: src_code.chars().peekable(),
			line: 0,
			start_line: 0,
			column: 0,
			start_column: 0,
			curr_src: String::new(),
			new_line: false,
		}
	}

	pub fn get_tokens(mut self) -> Result<Vec<Token>, Vec<LexerError>> {
		let mut tokens = Vec::new();
		let mut errors = Vec::new();

		let handlers = [
			Self::lex_symbol,
			Self::lex_string,
			Self::lex_number,
			Self::lex_identifier,
		];
		while self.chars.peek().is_some() {
			if self.lex_whitespace().is_some() {
				continue;
			}

			self.check_line_break();

			self.start_line = self.line;
			self.start_column = self.column;
			self.curr_src = String::new();

			let mut invalid = true;
			for handler in handlers {
				match handler(&mut self) {
					Ok(None) => (),
					Err(err) => {
						errors.push(err);
						invalid = false;
					}
					Ok(Some(value)) => {
						if !matches!(
							value,
							TokenValue::SingleLineComment(_) | TokenValue::MultiLineComment(_)
						) {
							tokens.push(Token {
								value,
								span: Span {
									start: Location {
										line: self.start_line,
										column: self.start_column,
									},
									end: Location {
										line: self.line,
										column: self.column,
									},
								},
								src: self.curr_src.clone(),
							});
						}

						invalid = false;
						break;
					}
				}
			}

			if invalid {
				errors.push(LexerError::invalid_character(
					self.get_char().unwrap(),
					Span {
						start: Location {
							line: self.start_line,
							column: self.start_column,
						},
						end: Location {
							line: self.start_line,
							column: self.start_column + 1,
						},
					},
				));
			}
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

	fn lex_whitespace(&mut self) -> Option<TokenValue> {
		let mut ret = None;
		let mut line_break_cnt = 0;
		while let Some(ch) = self.chars.peek() {
			if *ch == '\n' {
				line_break_cnt += 1;
			} else if ![' ', '\t'].contains(ch) {
				break;
			}
			self.get_char();
			ret = Some(TokenValue::Whitespace);
		}

		if line_break_cnt > 2 {
			ret = Some(TokenValue::EmptyLine);
		}

		ret
	}

	fn lex_number(&mut self) -> Result<Option<TokenValue>, LexerError> {
		let Some(ch) = self.chars.peek() else {
			return Ok(None);
		};
		if !ch.is_ascii_digit() {
			return Ok(None);
		}

		self.get_char();

		let mut point = false;
		while let Some(next_char) = self.chars.peek() {
			if next_char.is_ascii_digit() {
				self.get_char();
			} else if !point && *next_char == '.' {
				self.get_char();
				point = true;
			} else {
				break;
			}
		}

		if let Ok(number) = self.curr_src.parse() {
			Ok(Some(TokenValue::Number(number)))
		} else {
			Err(LexerError::invalid_number(
				self.curr_src.clone(),
				Span {
					start: Location {
						line: self.start_line,
						column: self.start_column,
					},
					end: Location {
						line: self.line,
						column: self.column,
					},
				},
			))
		}
	}

	fn lex_identifier(&mut self) -> Result<Option<TokenValue>, LexerError> {
		let Some(ch) = self.chars.peek() else {
			return Ok(None);
		};
		if !(is_xid_start(*ch) || *ch == '_') {
			return Ok(None);
		}

		self.get_char();

		while let Some(next_char) = self.chars.peek() {
			if is_xid_continue(*next_char) {
				self.get_char();
				continue;
			}
			break;
		}

		Ok(Some(Lexer::get_keyword(&self.curr_src)))
	}

	fn lex_string(&mut self) -> Result<Option<TokenValue>, LexerError> {
		Ok(self.parse_string()?.map(TokenValue::String))
	}

	fn parse_string(&mut self) -> Result<Option<String>, LexerError> {
		let Some(ch) = self.chars.peek() else {
			return Ok(None);
		};
		if *ch != '"' {
			return Ok(None);
		}
		self.get_char();

		let mut string = String::new();
		let mut terminated = false;
		let mut escape = false;
		let mut invalid_escape = None;
		while let Some(mut next_char) = self.get_char() {
			if escape {
				escape = false;
				next_char = match next_char {
					'"' => '"',
					'\\' => '\\',
					'n' => '\n',
					ch => {
						invalid_escape = Some(LexerError::invalid_escape(
							ch,
							Span {
								start: Location {
									line: self.line,
									column: self.column - 2,
								},
								end: Location {
									line: self.line,
									column: self.column,
								},
							},
						));
						next_char
					}
				}
			} else if next_char == '\\' {
				escape = true;
				continue;
			} else if next_char == '"' {
				terminated = true;
				break;
			}
			string.push(next_char);
		}

		if let Some(err) = invalid_escape {
			return Err(err);
		}

		if !terminated {
			Err(LexerError::unterminated_string(Span {
				start: Location {
					line: self.line,
					column: self.column,
				},
				end: Location {
					line: self.line,
					column: self.column + 1,
				},
			}))
		} else {
			Ok(Some(string))
		}
	}

	fn lex_symbol(&mut self) -> Result<Option<TokenValue>, LexerError> {
		let Some(ch1) = self.chars.peek().copied() else {
			return Ok(None);
		};
		let mut value = match ch1 {
			'+' => TokenValue::Plus,
			'-' => TokenValue::Minus,
			'*' => TokenValue::Asterisk,
			'/' => TokenValue::Slash,
			'%' => TokenValue::Percent,
			'.' => TokenValue::Point,
			':' => TokenValue::Colon,
			',' => TokenValue::Comma,
			';' => TokenValue::Semicolon,
			'!' => TokenValue::Bang,
			'?' => TokenValue::Question,
			'=' => TokenValue::Equal,
			'<' => TokenValue::Less,
			'>' => TokenValue::Greater,

			'(' => TokenValue::ParenOpen,
			')' => TokenValue::ParenClose,
			'[' => TokenValue::BracketOpen,
			']' => TokenValue::BracketClose,
			'{' => TokenValue::CurlyOpen,
			'}' => TokenValue::CurlyClose,

			_ => return Ok(None),
		};
		self.get_char();

		let Some(ch2) = self.chars.peek().copied() else {
			return Ok(Some(value));
		};
		value = match (ch1, ch2) {
			('/', '/') => {
				let mut comment = String::new();

				while let Some(ch) = self.get_char() {
					if ch == '\n' {
						break;
					}
					comment.push(ch);
				}

				TokenValue::SingleLineComment(comment)
			}
			('/', '*') => {
				let mut comment = String::new();

				let mut terminated = false;
				let mut asterisk = false;
				while let Some(ch) = self.get_char() {
					if ch == '*' {
						asterisk = true;
					} else if asterisk && ch == '/' {
						terminated = true;
						break;
					} else {
						asterisk = false;
					}
					comment.push(ch);
				}

				if !terminated {
					return Err(LexerError::unterminated_comment(Span {
						start: Location {
							line: self.line,
							column: self.column,
						},
						end: Location {
							line: self.line,
							column: self.column + 1,
						},
					}));
				}

				comment.pop(); // remove * at the end

				TokenValue::MultiLineComment(comment)
			}
			('<', '-') => TokenValue::ArrowLeft,
			(':', ':') => TokenValue::ColonColon,
			('?', '?') => TokenValue::QuestionQuestion,
			('=', '=') => TokenValue::EqualEqual,
			('!', '=') => TokenValue::BangEqual,
			('<', '=') => TokenValue::LessEqual,
			('>', '=') => TokenValue::GreaterEqual,

			_ => return Ok(Some(value)),
		};
		if !matches!(
			value,
			TokenValue::SingleLineComment(_) | TokenValue::MultiLineComment(_)
		) {
			self.get_char();
		}

		Ok(Some(value))
	}

	fn get_char(&mut self) -> Option<char> {
		let curr_char = self.chars.next();
		self.check_line_break();
		if let Some(char) = curr_char {
			self.curr_src.push(char);
			self.column += 1;
			if char == '\n' {
				self.new_line = true;
			}
		}

		curr_char
	}

	fn check_line_break(&mut self) {
		if self.new_line {
			self.new_line = false;
			self.line += 1;
			self.column = 0;
		}
	}

	fn get_keyword(id: &str) -> TokenValue {
		match id {
			"null" => TokenValue::Null,
			"true" => TokenValue::Bool(true),
			"false" => TokenValue::Bool(false),
			"global" => TokenValue::Global,
			"actions" => TokenValue::Actions,
			"function" => TokenValue::Function,
			"let" => TokenValue::Let,
			"and" => TokenValue::LAnd,
			"or" => TokenValue::LOr,
			"if" => TokenValue::If,
			"else" => TokenValue::Else,
			"while" => TokenValue::While,
			"continue" => TokenValue::Continue,
			"break" => TokenValue::Break,
			"return" => TokenValue::Return,
			"spawn" => TokenValue::Spawn,
			_ => TokenValue::Identifier(id.to_owned()),
		}
	}
}
