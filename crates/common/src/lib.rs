use serde::Serialize;

use ast::{Location, Span};

pub mod colored_string;
pub use colored_string::*;

pub fn check_src(src_code: String, target: Target) -> Status {
	if let Err(error) = parser::get_ast(&src_code) {
		Status {
			status: "error".to_owned(),
			errors: Some(convert_parser_error(&error, &src_code, target)),
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

pub fn convert_parser_error(error: &parser::Error, src_code: &str, target: Target) -> Vec<Error> {
	let mut ret = Vec::new();
	match error {
		parser::Error::Lexer(errors) => {
			for err in errors {
				ret.push(match err {
					parser::LexerError::InvalidCharacter { char, loc } => {
						let span = Span {
							start: *loc,
							end: Location {
								line: loc.line,
								column: loc.column + 1,
							},
						};

						Error {
							title: format!("Invalid character `{}` found!", char),
							pos: Some(Position {
								span,
								span_str: create_error_location(
									&span.start,
									&span.end,
									src_code,
									target,
								),
							}),
						}
					}
					parser::LexerError::InvalidNumber { src, span } => Error {
						title: format!("Invalid number `{}` found!", src),
						pos: Some(Position {
							span: *span,
							span_str: create_error_location(
								&span.start,
								&span.end,
								src_code,
								target,
							),
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
							title: "Unexpected end of file!".to_owned(),
							pos: Some(Position {
								span,
								span_str: create_error_location(
									&span.start,
									&span.end,
									src_code,
									target,
								),
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
								span_str: create_error_location(
									&span.start,
									&span.end,
									src_code,
									target,
								),
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
					span: *span,
					span_str: create_error_location(&span.start, &span.end, src_code, target),
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
						span: *span,
						span_str: create_error_location(&span.start, &span.end, src_code, target),
					}),
				}
			}
			parser::ParserError::SecondActions {
				actions1: _,
				actions2,
			} => Error {
				title: "Only one `actions` block is allowed!".to_owned(),
				pos: Some(Position {
					span: *actions2,
					span_str: create_error_location(
						&actions2.start,
						&actions2.end,
						src_code,
						target,
					),
				}),
			},
			parser::ParserError::ExpectedSemicolon { span } => Error {
				title: "Expected semicolon `;`!".to_owned(),
				pos: Some(Position {
					span: *span,
					span_str: create_error_location(&span.start, &span.end, src_code, target),
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
					title: "ERROR: Unexpected end of file!".to_owned(),
					pos: Some(Position {
						span,
						span_str: create_error_location(&span.start, &span.end, src_code, target),
					}),
				}
			}
		}),
	}

	ret
}

pub fn create_error_location(
	start: &Location,
	end: &Location,
	src: &str,
	target: Target,
) -> String {
	let mut ret = String::new();

	let mut lines: Vec<&str> = src.lines().collect();
	lines.push("");

	let pipe = ColoredString::blue("|", target);

	let number_padding = (end.line + 1).to_string().len();
	ret += &format!("{:>pad$}\n", pipe, pad = number_padding + 2);

	let show_lines = 3;
	let mut skipped = false;
	for line_number in start.line..=end.line {
		if (end.line - start.line) > show_lines * 2
			&& !(line_number - start.line < show_lines || end.line - line_number < show_lines)
		{
			if !skipped {
				skipped = true;
				ret += &format!(
					"{:>pad$} {}\n",
					pipe,
					ColoredString::blue("...", target),
					pad = number_padding + 2
				);
			}
		} else if let Some(line) = lines.get(line_number) {
			let line = match target {
				Target::None => line.to_owned().to_owned(),
				Target::ANSI => line.to_owned().to_owned(),
				Target::HTML => html_escape::encode_text(line).to_string(),
			};
			ret += &format!(
				"{:<pad$} {:} {}\n",
				ColoredString::blue((line_number + 1).to_string(), target),
				pipe,
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
			pipe,
			" ".repeat(start.column),
			ColoredString::red("^".repeat(end.column - start.column), target),
			pad = number_padding + 2
		);
	}

	ret += &format!("{:>pad$}", pipe, pad = number_padding + 2);

	ret
}
