use ast::Span;

#[derive(Debug)]
pub enum ParserError {
	Fatal(String),
	Positional { msg: String, span: Span },
	UnexpectedToken { src: String, span: Span },
	SecondActions { actions1: Span, actions2: Span },
	UnexpectedEoF,
}
