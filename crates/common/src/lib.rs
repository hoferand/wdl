use serde::Serialize;

use ast::{Location, Span};

pub mod colored_string;
pub use colored_string::*;

pub fn check_src(src_code: String, target: ColorMode) -> Status {
	if let Err(error) = parser::get_ast(&src_code) {
		Status {
			status: "Error".to_owned(),
			errors: Some(convert_parser_error(&error, &src_code, target)),
		}
	} else {
		Status {
			status: "Ok".to_owned(),
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

pub fn convert_parser_error(
	error: &parser::Error,
	src_code: &str,
	target: ColorMode,
) -> Vec<Error> {
	let mut ret = Vec::new();
	match error {
		parser::Error::Lexer(errors) => {
			for err in errors {
				let title = match &err.kind {
					parser::LexerErrorKind::InvalidCharacter { char } => {
						format!("Invalid character `{}` found!", char)
					}
					parser::LexerErrorKind::InvalidNumber { src } => {
						format!("Invalid number `{}` found!", src)
					}
					parser::LexerErrorKind::InvalidEscape { char } => {
						format!("Invalid character escape `\\{}`!", char)
					}
					parser::LexerErrorKind::UnterminatedString => "Missing `\"`!".to_owned(),
					parser::LexerErrorKind::UnterminatedComment => "Missing `*/`!".to_owned(),
				};

				ret.push(Error {
					title,
					pos: Some(Position {
						span: err.span,
						span_str: create_error_location(
							&err.span.start,
							&err.span.end,
							src_code,
							target,
						),
					}),
				})
			}
		}
		parser::Error::Parser(err) => {
			let mut span = err.span;
			let title = match &err.kind {
				parser::ParserErrorKind::UnexpectedToken { src, expected } => {
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
					msg.push('!');
					msg
				}
				parser::ParserErrorKind::SecondActions {
					actions1: _,
					actions2: _,
				} => "Only one `actions` block is allowed!".to_owned(),
				parser::ParserErrorKind::ExpectedSemicolon => "Expected semicolon `;`!".to_owned(),
				parser::ParserErrorKind::UnexpectedEoF { expected: _ } => {
					let last_line = src_code.lines().last().unwrap_or("").to_owned();
					let start = Location {
						line: src_code.lines().count() - 1,
						column: last_line.chars().count(),
					};
					span = Some(Span {
						start,
						end: Location {
							line: start.line,
							column: start.column + 1,
						},
					});

					"Unexpected end of file!".to_owned()
				}
				parser::ParserErrorKind::NoActions => "No actions block found!".to_owned(),
				parser::ParserErrorKind::ScopedIdentifierAssign { id } => {
					format!("Assignment to scoped identifier `{}` is not allowed!", id)
				}
				parser::ParserErrorKind::UnexpectedContinue => {
					"`continue` is only allowed inside loops!".to_owned()
				}
				parser::ParserErrorKind::UnexpectedBreak => {
					"`break` is only allowed inside loops!".to_owned()
				}
				parser::ParserErrorKind::UnexpectedReturn => {
					"`return` is only allowed inside function bodies!".to_owned()
				}
				parser::ParserErrorKind::DuplicateParameter { id } => {
					format!("Duplicate parameter `{}` found!", id)
				}
				parser::ParserErrorKind::DuplicateArgument { id } => {
					format!("Duplicate argument `{}` found!", id)
				}
				parser::ParserErrorKind::ScopedArgument { id } => {
					format!(
						"Scoped identifier `{}` is not allowed as argument name!",
						id
					)
				}
				parser::ParserErrorKind::PositionalAfterNamed => {
					"Positional arguments are not allowed after named arguments!".to_owned()
				}
			};
			ret.push(Error {
				title,
				pos: span.map(|span| Position {
					span,
					span_str: create_error_location(&span.start, &span.end, src_code, target),
				}),
			});
		}
	}

	ret
}

pub fn create_error_location(
	start: &Location,
	end: &Location,
	src: &str,
	target: ColorMode,
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
				ColorMode::None => line.to_owned().to_owned(),
				ColorMode::ANSI => line.to_owned().to_owned(),
				ColorMode::HTML => html_escape::encode_text(line).to_string(),
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
