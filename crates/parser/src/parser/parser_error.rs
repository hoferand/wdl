use ast::Span;

#[derive(Debug)]
pub enum ParserError {
	Fatal(String),
	UnexpectedToken { src: String, span: Span },
	SecondOrder { span: Span },
	UnexpectedEoF,
}
