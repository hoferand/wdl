use ast::{Location, Span};
use serde::Serialize;

pub fn check_src(src_code: String) -> Status {
	if let Err(error) = parser::get_ast(&src_code) {
		Status {
			status: "error".to_owned(),
			errors: Some(convert_parser_error(error, &src_code)),
		}
	} else {
		Status {
			status: "ok".to_owned(),
			errors: None,
		}
	}
}

#[derive(Debug, Serialize)]
pub struct Status {
	pub status: String,
	pub errors: Option<Vec<Error>>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub struct Error {
	pub title: String,
	pub pos: Option<Position>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub struct Position {
	pub span: Span,
	pub span_str: String,
}

fn convert_parser_error(error: parser::Error, src_code: &str) -> Vec<Error> {
	let mut ret = Vec::new();
	match error {
		parser::Error::Lexer(errors) => {
			for err in errors {
				ret.push(match err {
					parser::LexerError::InvalidCharacter { char, loc } => {
						let span = Span {
							start: loc,
							end: Location {
								line: loc.line,
								column: loc.column + 1,
							},
						};

						Error {
							title: format!("Invalid character `{}` found!", char),
							pos: Some(Position {
								span,
								span_str: create_error_location(&span.start, &span.end, src_code),
							}),
						}
					}
					parser::LexerError::InvalidNumber { src, span } => Error {
						title: format!("Invalid number `{}` found!", src),
						pos: Some(Position {
							span,
							span_str: create_error_location(&span.start, &span.end, src_code),
						}),
					},
					parser::LexerError::UnexpectedEndOfFile => {
						let last_line = src_code.lines().last().unwrap_or("").to_owned();
						let start = Location {
							line: src_code.lines().count() - 1,
							column: last_line.chars().count(),
						};
						let span = Span {
							start,
							end: Location {
								line: start.line,
								column: start.column + 1,
							},
						};

						Error {
							title: format!("Unexpected end of file!"),
							pos: Some(Position {
								span,
								span_str: create_error_location(&span.start, &span.end, src_code),
							}),
						}
					}
					parser::LexerError::InvalidEscape { char, loc } => {
						let span = Span {
							start: Location {
								line: loc.line,
								column: loc.column - 2,
							},
							end: Location {
								line: loc.line,
								column: loc.column,
							},
						};

						Error {
							title: format!("Invalid character escape `\\{}`!", char),
							pos: Some(Position {
								span,
								span_str: create_error_location(&span.start, &span.end, src_code),
							}),
						}
					}
				})
			}
		}
		parser::Error::Parser(err) => ret.push(match err {
			parser::ParserError::Fatal(msg) => Error {
				title: format!("{}!", msg),
				pos: None,
			},
			parser::ParserError::Positional { msg, span } => Error {
				title: format!("{}!", msg),
				pos: Some(Position {
					span,
					span_str: create_error_location(&span.start, &span.end, src_code),
				}),
			},
			parser::ParserError::UnexpectedToken {
				src,
				span,
				expected,
			} => {
				let mut msg = format!("Unexpected token `{}`", src);
				if expected.len() == 1 {
					msg += &format!(", expected `{}`", expected[0]);
				} else if !expected.is_empty() {
					msg += ", expected one of [";
					msg += &expected
						.iter()
						.map(|e| format!("`{}`", e))
						.collect::<Vec<String>>()
						.join(", ");
					msg += "]";
				}
				Error {
					title: format!("{}!", msg),
					pos: Some(Position {
						span,
						span_str: create_error_location(&span.start, &span.end, src_code),
					}),
				}
			}
			parser::ParserError::SecondActions {
				actions1: _,
				actions2,
			} => Error {
				title: format!("Only one `actions` block is allowed!"),
				pos: Some(Position {
					span: actions2,
					span_str: create_error_location(&actions2.start, &actions2.end, src_code),
				}),
			},
			parser::ParserError::ExpectedSemicolon { span } => Error {
				title: format!("Expected semicolon `;`!"),
				pos: Some(Position {
					span,
					span_str: create_error_location(&span.start, &span.end, src_code),
				}),
			},
			parser::ParserError::UnexpectedEoF => {
				let last_line = src_code.lines().last().unwrap_or("").to_owned();
				let start = Location {
					line: src_code.lines().count() - 1,
					column: last_line.chars().count(),
				};
				let span = Span {
					start,
					end: Location {
						line: start.line,
						column: start.column + 1,
					},
				};

				Error {
					title: format!("ERROR: Unexpected end of file!"),
					pos: Some(Position {
						span,
						span_str: create_error_location(&span.start, &span.end, src_code),
					}),
				}
			}
		}),
	}

	ret
}

fn create_error_location(start: &Location, end: &Location, src: &str) -> String {
	let mut ret = String::new();

	let mut lines: Vec<&str> = src.lines().collect();
	lines.push("");

	let number_padding = (end.line + 1).to_string().len();
	ret += &format!("{:>pad$}\n", "|", pad = number_padding + 2);

	let show_lines = 3;
	let mut skipped = false;
	for line_number in start.line..=end.line {
		if (end.line - start.line) > show_lines * 2
			&& !(line_number - start.line < show_lines || end.line - line_number < show_lines)
		{
			if !skipped {
				skipped = true;
				ret += &format!("{:>pad$} {}\n", "|", "...", pad = number_padding + 2);
			}
		} else if let Some(line) = lines.get(line_number) {
			ret += &format!(
				"{:<pad$} {:} {}\n",
				(line_number + 1).to_string(),
				"|",
				line,
				pad = number_padding,
			);
		} else {
			ret = format!(
				"Internal error, line number `{}` does not exist!",
				line_number + 1
			);
			return ret;
		}
	}

	if start.line == end.line {
		// TODO: fix tab ident
		ret += &format!(
			"{:>pad$} {}{}\n",
			"|",
			" ".repeat(start.column),
			"^".repeat(end.column - start.column),
			pad = number_padding + 2
		);
	}

	ret += &format!("{:>pad$}", "|", pad = number_padding + 2);

	ret
}
