use ast::Span;

#[derive(Debug)]
pub enum ParserError {
	Fatal(String),
	UnexpectedStatement { name: String, span: Span },
	UnexpectedToken { src: String, span: Span },
	UnexpectedEoF,
}
