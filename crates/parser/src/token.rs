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
	Question,         // ?
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
	LAnd, // and
	LOr,  // or
	If,
	Else,
	While,
	Continue,
	Break,
	Return,
	Spawn,

	// Needed for auto formatting
	Whitespace,
	EmptyLine,
	SingleLineComment(String),
	MultiLineComment(String),

	EoF,
}

impl TokenValue {
	pub fn get_type(&self) -> String {
		match self {
			TokenValue::Null => "null",
			TokenValue::Bool(_) => "<bool>",
			TokenValue::Number(_) => "<number>",
			TokenValue::String(_) => "<string>",
			TokenValue::Identifier(_) => "<identifier>",
			TokenValue::Plus => "+",
			TokenValue::Minus => "-",
			TokenValue::Asterisk => "*",
			TokenValue::Slash => "/",
			TokenValue::Percent => "%",
			TokenValue::Question => "?",
			TokenValue::QuestionQuestion => "??",
			TokenValue::Point => ".",
			TokenValue::Colon => ":",
			TokenValue::ColonColon => "::",
			TokenValue::Bang => "!",
			TokenValue::Comma => ",",
			TokenValue::Semicolon => ";",
			TokenValue::Equal => "=",
			TokenValue::EqualEqual => "==",
			TokenValue::BangEqual => "!=",
			TokenValue::Less => "<",
			TokenValue::LessEqual => "<=",
			TokenValue::Greater => ">",
			TokenValue::GreaterEqual => "=>",
			TokenValue::ArrowLeft => "<-",
			TokenValue::ParenOpen => "(",
			TokenValue::ParenClose => ")",
			TokenValue::BracketOpen => "[",
			TokenValue::BracketClose => "]",
			TokenValue::CurlyOpen => "{",
			TokenValue::CurlyClose => "}",
			TokenValue::Global => "global",
			TokenValue::Actions => "actions",
			TokenValue::Function => "function",
			TokenValue::Let => "let",
			TokenValue::LAnd => "and",
			TokenValue::LOr => "or",
			TokenValue::If => "if",
			TokenValue::Else => "else",
			TokenValue::While => "while",
			TokenValue::Continue => "continue",
			TokenValue::Break => "break",
			TokenValue::Return => "return",
			TokenValue::Spawn => "spawn",
			TokenValue::EoF => "<eof>",
			TokenValue::Whitespace => "<whitespace>",
			TokenValue::EmptyLine => "<empty_line>",
			TokenValue::SingleLineComment(_) => "<single_line_comment>",
			TokenValue::MultiLineComment(_) => "<multi_line_comment>",
		}
		.to_owned()
	}
}
