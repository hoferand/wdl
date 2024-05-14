# Syntax

## Notation

| Notation     | Example              | Description                            |
| ------------ | -------------------- | -------------------------------------- |
| `TypeWriter` | `if`                 | The exact character(s)                 |
| _Italic_     | _Identifier_         | A syntactical production               |
| x ::= y      | _Null_ ::= `null`    | Definition of a syntactical production |
| x y          | `spawn` _Expression_ | Concatenation of x and y               |
| x?           | _Expression_?        | The item x is optional                 |
| x\*          | _Char_\*             | 0 or more occurrences of x             |
| x+           | _Digit_+             | 1 or more occurrences of x             |
| x \| y       | `true` \| `false`    | Either x or y                          |
| [ x - y ]    | [ `0` - `9` ]        | One of the characters in the range     |
| ( x y )      | ( _Value_ `,` )\*    | Group multiple items                   |

## Identifier

Identifiers used for variables, channels, object members, and function names comply with the [Unicode Standard Annex #31](http://www.unicode.org/reports/tr31/).

_Identifier_ ::= _Start_ _Continue_\*

Where _Start_ is a character from the Unicode set `XID_Start` or the underscore character `*`, and _Continue_ is a character of the set `XID_Continue`.

**Example**

```wdl
var1

ladungsträger

_12
```

Furthermore, scoped identifiers are used to identify variables and functions from the standard library.

_ScopedIdentifier_ ::= ( _Identifier_ `::` )\* _Identifier_

**Example**

```wdl
action::pickup

log::info
```

## Value

_Value_ can be one of:

| Syntax                                                     | Name   |
| ---------------------------------------------------------- | ------ |
| `null`                                                     | Null   |
| `true` \| `false`                                          | Bool   |
| _Digit_+ (`.` _Digit_+)?                                   | Number |
| `"` _Char_\* `"`                                           | String |
| `[` ( _Value_ `,` )\* `]`                                  | Array  |
| `{` ( ( _Identifier_ \| _String_ ) `:` _Value_ `,` )\* `}` | Object |

_Digit_ ::= [ `0` - `9` ]

_Char_ can be any valid Unicode character, with the exception that `"` and `\` must be escaped by a preceding `\`. Furthermore, we allow using `\n` inside strings for adding line breaks.

## Expression

_Expression_ can be one of:

| Syntax                                                            | Name              |
| ----------------------------------------------------------------- | ----------------- |
| _Value_                                                           | Value             |
| _ScopedIdentifier_                                                | Variable          |
| `<-` _Expression_                                                 | Receive           |
| _UnaryOperator_ _Expression_                                      | Unary expression  |
| _Expression_ _BinaryOperator_ _Expression_                        | Binary expression |
| `(` _Expression_ `)`                                              | Group             |
| _Expression_ `[` _Expression_ `]`                                 | Index             |
| _Expression_ `.` _Identifier_                                     | Member            |
| _Expression_ `(` ( ( _Identifier_ `:` )? _Expression_ `,` )\* `)` | Call              |
| `spawn` _Expression_                                              | Spawn             |

_UnaryOperator_ ::= `-` | `!`

_BinaryOperator_ ::= `+`| `-` | `*` | `/` | `%` | `??` | `==` | `!=` | `<` | `<=` | `>` | `>=` | `and` | `or`

## Statement

_Statement_ can be one of:

| Syntax                                                                                        | Name        |
| --------------------------------------------------------------------------------------------- | ----------- |
| _Expression_ `;`                                                                              | Expression  |
| `let` _Identifier_ `=` _Expression_                                                           | Declaration |
| _Identifier_ `=` _Expression_                                                                 | Assignment  |
| _Expression_ `<-` _Expression_                                                                | Send        |
| `if` _Expression_ `{` _Statement_\* `}` ( `else` `{` _Statement_\* `}` \| `else` _If-else_ )? | If-else     |
| `while` _Expression_ `{` _Statement_\* `}`                                                    | While       |
| `return` _Expression_? `;`                                                                    | Return      |
| `break` `;`                                                                                   | Break       |
| `continue` `;`                                                                                | Continue    |

## Workflow

_Workflow_ ::= _GlobalDeclaration_\* `actions` `{` _Statement_\* `}` _GlobalDeclaration_\*

_GlobalDeclaration_ can be one of:

| Syntax                                                                       | Name            |
| ---------------------------------------------------------------------------- | --------------- |
| `global` _Identifier_ `=` _Expression_ `;`                                   | Global Variable |
| `function` _Identifier_ `(` ( _Identifier_ `,` )\* `)` `{` _Statement_\* `}` | Function        |

## Comment

To document code, comments can be used. Comments can be placed anywhere in the code except within string literals. Two types of comments are supported. Firstly, single-line comments, which start with `//` and comment out the rest of the line. Secondly, multi-line comments, which begin with `/*` and end with `*/`, commenting out everything between them. Nesting multi-line comments is not supported; thus, `/* /* */ */` raises a syntax error because the comment ends with the first `*/`, and the second occurrence is unexpected.
