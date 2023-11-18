use ast::Span;

#[derive(Debug)]
pub enum ParserError {
	Fatal(String),
	UnexpectedToken { src: String, span: Span },
	SecondOrder { order1: Span, order2: Span },
	UnexpectedEoF,
}
