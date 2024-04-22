use ast::Span;

#[derive(Debug)]
pub enum ParserError {
	Fatal(String),
	Positional {
		msg: String,
		span: Span,
	},
	UnexpectedToken {
		src: String,
		span: Span,
		expected: Vec<String>,
	},
	SecondActions {
		actions1: Span,
		actions2: Span,
	},
	ExpectedSemicolon {
		span: Span,
	},
	UnexpectedEoF,
}
