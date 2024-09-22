//! This library provides common functions to format errors from the parser
//! and interpreter crate into user-friendly messages.

use serde::Serialize;

use ast::{Location, Span};

mod colored_string;
pub use colored_string::*;

/// User-readable representation for different kinds of errors.
///
/// CAUTION: The `title` is not escaped in any way, so it is not HTML safe!
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub struct Error {
	pub title: String,
	pub pos: Option<Position>,
}

/// Contains the structured representation of a span
/// as well its human readable version.  
///
/// CAUTION: The `span_str` is only HTML safe if generated
/// with the `HTML` color mode.
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub struct Position {
	pub span: Span,
	pub span_str: String,
}

pub fn format_parser_error(error: &parser::Error, src_code: &str, mode: ColorMode) -> Vec<Error> {
	let mut ret = Vec::new();
	match error {
		parser::Error::Lexer(errors) => {
			for err in errors {
				let title = match &err.kind {
					parser::LexerErrorKind::InvalidCharacter { char } => {
						format!("Invalid character `{}` found!", char)
					}
					parser::LexerErrorKind::InvalidEscape { char } => {
						format!("Invalid character escape `\\{}`!", char)
					}
					parser::LexerErrorKind::InvalidNumber { src } => {
						format!("Invalid number `{}` found!", src)
					}
					parser::LexerErrorKind::UnterminatedComment => "Missing `*/`!".to_owned(),
					parser::LexerErrorKind::UnterminatedString => "Missing `\"`!".to_owned(),
				};

				ret.push(Error {
					title,
					pos: Some(Position {
						span: err.span,
						span_str: format_span(&err.span.start, &err.span.end, src_code, mode),
					}),
				})
			}
		}
		parser::Error::Parser(err) => {
			let mut span = err.span;
			let title = match &err.kind {
				parser::ParserErrorKind::DuplicateArgument { id } => {
					format!("Duplicate argument `{}` found!", id)
				}
				parser::ParserErrorKind::DuplicateParameter { id } => {
					format!("Duplicate parameter `{}` found!", id)
				}
				parser::ParserErrorKind::ExpectedSemicolon => "Expected semicolon `;`!".to_owned(),
				parser::ParserErrorKind::NoActions => "No actions block found!".to_owned(),
				parser::ParserErrorKind::PositionalAfterNamed => {
					"Positional arguments are not allowed after named arguments!".to_owned()
				}
				parser::ParserErrorKind::ScopedArgument { id } => {
					format!(
						"Scoped identifier `{}` is not allowed as argument name!",
						id
					)
				}
				parser::ParserErrorKind::ScopedIdentifierAssign { id } => {
					format!("Assignment to scoped identifier `{}` is not allowed!", id)
				}
				parser::ParserErrorKind::SecondActions {
					actions1: _,
					actions2: _,
				} => "Only one `actions` block is allowed!".to_owned(),
				parser::ParserErrorKind::UnexpectedBreak => {
					"`break` is only allowed inside loops!".to_owned()
				}
				parser::ParserErrorKind::UnexpectedContinue => {
					"`continue` is only allowed inside loops!".to_owned()
				}
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
				parser::ParserErrorKind::UnexpectedReturn => {
					"`return` is only allowed inside function bodies!".to_owned()
				}
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
			};

			ret.push(Error {
				title,
				pos: span.map(|span| Position {
					span,
					span_str: format_span(&span.start, &span.end, src_code, mode),
				}),
			});
		}
	}

	ret
}

// This function is only available with the `interpreter` feature flag
// because the interpreter crate is not compatible with WASM.
#[cfg(feature = "interpreter")]
pub fn format_interpreter_error(
	error: &interpreter::Error,
	src_code: &str,
	mode: ColorMode,
) -> Error {
	let title = match &error.kind {
		interpreter::ErrorKind::ArityMismatch { expected, given } => {
			format!(
				"Invalid count of function call parameter, expected `{}`, given `{}`!",
				expected, given
			)
		}
		interpreter::ErrorKind::DivisionByZero => "Division by zero!".to_owned(),
		interpreter::ErrorKind::Fatal(msg) => format!("{}!", msg),
		interpreter::ErrorKind::InvalidType { msg } => {
			format!("Invalid types, {}!", msg)
		}
		interpreter::ErrorKind::MissingArgument { id } => {
			format!("Argument `{}` missing!", id)
		}
		interpreter::ErrorKind::OrderCancel => "Order canceled!".to_owned(),
		interpreter::ErrorKind::OrderDone => "Order done!".to_owned(),
		interpreter::ErrorKind::UnknownArgument { id } => {
			format!("Named argument `{}` unknown!", id)
		}
		interpreter::ErrorKind::VariableAlreadyInUse { id } => {
			format!("Variable `{}` already in use!", id.id)
		}
		interpreter::ErrorKind::VariableNotFound { id } => {
			format!("Variable `{}` not found!", id)
		}
	};

	Error {
		title,
		pos: error.span.map(|span| Position {
			span,
			span_str: format_span(&span.start, &span.end, src_code, mode),
		}),
	}
}

/// Returns a string showing the precise error location inside the source code.  
///
/// CAUTION: The returned string is only HTML safe if called with the `HTML` color mode.
pub fn format_span(start: &Location, end: &Location, src: &str, mode: ColorMode) -> String {
	let mut ret = String::new();

	let mut lines: Vec<&str> = src.lines().collect();
	lines.push("");

	let pipe = ColoredString::blue("|", mode);

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
					ColoredString::blue("...", mode),
					pad = number_padding + 2
				);
			}
		} else if let Some(line) = lines.get(line_number) {
			let line = if mode == ColorMode::HTML {
				html_escape::encode_text(line).to_string()
			} else {
				line.to_owned().to_owned()
			};
			ret += &format!(
				"{:<pad$} {:} {}\n",
				ColoredString::blue((line_number + 1).to_string(), mode),
				pipe,
				line,
				pad = number_padding,
			);
		} else {
			return format!(
				"Internal error, line number `{}` does not exist!",
				line_number + 1
			);
		}
	}

	if start.line == end.line {
		// TODO: fix tab ident
		ret += &format!(
			"{:>pad$} {}{}\n",
			pipe,
			" ".repeat(start.column),
			ColoredString::red("^".repeat(end.column - start.column), mode),
			pad = number_padding + 2
		);
	}

	ret += &format!("{:>pad$}", pipe, pad = number_padding + 2);

	ret
}
