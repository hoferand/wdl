use ast::Span;

#[derive(Debug, Clone)]
pub struct Token {
	pub value: TokenValue,
	pub span: Span,
	pub src: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
	// Literals
	Null,
	Bool(bool),
	Number(f64),
	String(String),

	// Identifier
	Identifier(String),

	// Symbols
	Plus,             // +
	Minus,            // -
	Asterisk,         // *
	Slash,            // /
	Percent,          // %
	QuestionQuestion, // ??
	Point,            // .
	Colon,            // :
	ColonColon,       // ::
	Bang,             // !
	Comma,            // ,
	Semicolon,        // ;
	Equal,            // =
	EqualEqual,       // ==
	BangEqual,        // !=
	Less,             // <
	LessEqual,        // <=
	Greater,          // >
	GreaterEqual,     // >=
	ArrowLeft,        // <-

	// Brackets
	ParenOpen,    // (
	ParenClose,   // )
	BracketOpen,  // [
	BracketClose, // ]
	CurlyOpen,    // {
	CurlyClose,   // }

	// Keywords
	Global,
	Actions,
	Function,
	Let,
	Par,
	LAnd, // and
	LOr,  // or
	If,
	Else,
	While,
	Continue,
	Break,
	Return,
	Spawn,

	EoF,

	// Needed for auto formatting
	Whitespace,
	EmptyLine,
	SingleLineComment(String),
	MultiLineComment(String),
}
