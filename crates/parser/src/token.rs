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
	Star,             // *
	Slash,            // /
	Percent,          // %
	Hash,             // #
	At,               // @
	Pipe,             // |
	Point,            // .
	Question,         // ?
	QuestionQuestion, // ??
	Bang,             // !
	Comma,            // ,
	Colon,            // :
	ColonColon,       // ::
	Semicolon,        // ;
	Equal,            // =
	EqualEqual,       // ==
	BangEqual,        // !=
	Less,             // <
	LessEqual,        // <=
	Greater,          // >
	GreaterEqual,     // >=
	Arrow,            // ->

	// Brackets
	ParenOpen,    // (
	ParenClose,   // )
	BracketOpen,  // [
	BracketClose, // ]
	CurlyOpen,    // {
	CurlyClose,   // }

	// Keywords
	Use,
	Mod,
	Import,
	Global,
	Order,
	Fn,
	Let,
	Type,
	Par,
	Sleep,
	Print,
	LAnd, // and
	LOr,  // or
	If,
	Else,
	While,
	For,
	Continue,
	Break,
	Return,

	// Types
	TVoid,   // void
	TBool,   // bool
	TNumber, // number
	TString, // string
	TAny,    // any

	EoF,
}
