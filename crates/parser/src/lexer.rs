pub mod lexer_error;
pub use lexer_error::LexerError;

use std::{iter::Peekable, str::Chars};

use regex::Regex;
use reqwest::Url;

use ast::{Location, Span};

use crate::{Token, TokenValue};

pub(crate) struct Lexer<'c> {
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
						invalid = false;
						break;
					}
				}
			}

			if invalid {
				errors.push(LexerError::InvalidCharacter {
					char: self.get_char().unwrap(),
					loc: Location {
						line: self.start_line,
						column: self.start_column,
					},
				});
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

	fn lex_whitespace(&mut self) -> Option<()> {
		// TODO: detect empty lines, necessary for auto formatting
		let mut ret = None;
		while [Some(&' '), Some(&'\t'), Some(&'\n')].contains(&self.chars.peek()) {
			self.get_char();
			ret = Some(());
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
			Err(LexerError::InvalidNumber {
				src: self.curr_src.clone(),
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
			})
		}
	}

	fn lex_identifier(&mut self) -> Result<Option<TokenValue>, LexerError> {
		let Some(ch) = self.chars.peek() else {
			return Ok(None);
		};
		if !ch.is_ascii_alphabetic() || *ch == '_' {
			return Ok(None);
		}

		self.get_char();

		let mut semantic_string = false;
		while let Some(next_char) = self.chars.peek() {
			if *next_char == '"' {
				semantic_string = true;
			} else if next_char.is_ascii_alphanumeric() || *next_char == '_' {
				self.get_char();
				continue;
			}
			break;
		}

		if semantic_string {
			match self.curr_src.as_str() {
				"r" => {
					if let Some(string) = self.parse_string()? {
						if let Err(err) = Regex::new(&string) {
							return Err(LexerError::ExternalError {
								src: self.curr_src.clone(),
								msg: err.to_string(),
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
							});
						} else {
							return Ok(Some(TokenValue::String(string)));
						}
					};
				}
				"u" => {
					if let Some(string) = self.parse_string()? {
						let url = match Url::parse(&string) {
							Err(err) => {
								return Err(LexerError::ExternalError {
									src: self.curr_src.clone(),
									msg: err.to_string(),
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
								});
							}
							Ok(u) => u,
						};

						if !["http", "https"].contains(&url.scheme()) {
							return Err(LexerError::ExternalError {
								src: self.curr_src.clone(),
								msg: "Only 'http://' and 'https://' schemas are allowed!"
									.to_owned(),
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
							});
						}

						return Ok(Some(TokenValue::String(string)));
					};
				}
				"s" => todo!(),
				"a" => todo!(),
				_ => (),
			}
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

		// TODO: handle escaped characters: \n, \\, ...
		let mut string = String::new();
		let mut terminated = false;
		while let Some(next_char) = self.get_char() {
			if next_char == '"' {
				terminated = true;
				break;
			} else if next_char == '\n' {
				// TODO: rethink if multiline strings should be allowed
				break;
			}
			string.push(next_char);
		}

		if !terminated {
			Err(LexerError::UnexpectedEndOfString {
				src: self.curr_src.clone(),
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
			})
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
			'*' => TokenValue::Star,
			'/' => TokenValue::Slash,
			'%' => TokenValue::Percent,
			'#' => TokenValue::Hash,
			'@' => TokenValue::At,
			'|' => TokenValue::Pipe,
			'.' => TokenValue::Point,
			':' => TokenValue::Colon,
			',' => TokenValue::Comma,
			';' => TokenValue::Semicolon,
			'?' => TokenValue::Question,
			'!' => TokenValue::Bang,
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
			('-', '>') => TokenValue::ArrowRight,
			('<', '-') => TokenValue::ArrowLeft,
			('/', '/') => todo!("implement comments"),
			('/', '*') => todo!("implement comments"),
			(':', ':') => TokenValue::ColonColon,
			('?', '?') => TokenValue::QuestionQuestion,
			('=', '=') => TokenValue::EqualEqual,
			('!', '=') => TokenValue::BangEqual,
			('<', '=') => TokenValue::LessEqual,
			('>', '=') => TokenValue::GreaterEqual,

			_ => return Ok(Some(value)),
		};
		self.get_char();

		Ok(Some(value))
	}

	fn get_char(&mut self) -> Option<char> {
		let curr_char = self.chars.next();
		if self.new_line {
			self.new_line = false;
			self.line += 1;
			self.column = 0;
		}
		if let Some(char) = curr_char {
			self.curr_src.push(char);
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
			"actions" => TokenValue::Actions,
			"function" => TokenValue::Function,
			"let" => TokenValue::Let,
			"type" => TokenValue::Type,
			"par" => TokenValue::Par,
			"and" => TokenValue::LAnd,
			"or" => TokenValue::LOr,
			"if" => TokenValue::If,
			"else" => TokenValue::Else,
			"while" => TokenValue::While,
			"for" => TokenValue::For,
			"continue" => TokenValue::Continue,
			"break" => TokenValue::Break,
			"return" => TokenValue::Return,
			"void" => TokenValue::TVoid,
			"bool" => TokenValue::TBool,
			"number" => TokenValue::TNumber,
			"string" => TokenValue::TString,
			"any" => TokenValue::TAny,
			_ => TokenValue::Identifier(id.to_owned()),
		}
	}
}
