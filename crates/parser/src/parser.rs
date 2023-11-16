pub mod parser_error;
pub use parser_error::ParserError;

use ast::{
	Assignment, Binary, BinaryOperator, Block, Break, Continue, Else, Expression,
	FunctionDeclaration, GlobalDeclaration, Identifier, IdentifierFull, IdentifierTyped, If,
	Import, Let, Literal, Logical, LogicalOperator, Node, Order, Par, Print, Return, Sleep, Span,
	Statement, Type, Unary, UnaryOperator, While, Workflow,
};

use crate::{Token, TokenValue};

pub(crate) struct Parser<'t> {
	tokens: &'t [Token],
	pointer: usize,
	in_order: u32,
	in_function: u32,
	in_loop: u32,
	in_par: u32,
}

impl<'t> Parser<'t> {
	pub fn new(tokens: &'t [Token]) -> Self {
		Self {
			tokens: tokens,
			pointer: 0,
			in_order: 0,
			in_function: 0,
			in_loop: 0,
			in_par: 0,
		}
	}

	pub fn parse(mut self) -> Result<Workflow, ParserError> {
		let mut imports = Vec::new();
		let mut globals = Vec::new();
		let mut wf_order = None;
		let mut functions = Vec::new();

		// parse
		while let Some(stmt) = self.parse_statement()? {
			match stmt {
				Statement::Import(import) => imports.push(import),
				Statement::GlobalDeclaration(global) => globals.push(global),
				Statement::Order(order) if wf_order.is_none() => wf_order = Some(order),
				Statement::FunctionDeclaration(fn_) => functions.push(fn_),
				_ => {
					return Err(ParserError::UnexpectedStatement {
						name: stmt.get_type(),
						span: stmt.get_span().clone(),
					})
				}
			}
		}

		let Some(order) = wf_order else {
			return Err(ParserError::Fatal("No order block".to_owned()));
		};

		Ok(Workflow {
			imports,
			globals,
			order,
			functions,
		})
	}

	fn token_peek(&mut self) -> Option<&Token> {
		self.tokens.get(self.pointer)
	}

	fn token_next(&mut self) -> Option<&Token> {
		let token = self.tokens.get(self.pointer);
		self.pointer += 1;

		token
	}

	fn token_expect(&mut self, expect: TokenValue) -> Result<&Token, ParserError> {
		if let Some(token) = self.token_next() {
			if token.value != expect {
				Err(ParserError::UnexpectedToken {
					src: token.src.clone(),
					span: token.span.clone(),
				})
			} else {
				Ok(token)
			}
		} else {
			Err(ParserError::UnexpectedEoF)
		}
	}

	fn token_want(&mut self, want: TokenValue) -> Option<&Token> {
		let token = self.tokens.get(self.pointer)?;
		if token.value == want {
			self.pointer += 1;
			return Some(token);
		}

		None
	}

	fn token_next_comp_op(&mut self) -> Option<Node<BinaryOperator>> {
		let token = self.tokens.get(self.pointer)?;

		let op = match token.value {
			TokenValue::EqualEqual => BinaryOperator::Equal,
			TokenValue::BangEqual => BinaryOperator::NotEqual,
			TokenValue::Less => BinaryOperator::Less,
			TokenValue::LessEqual => BinaryOperator::LessEqual,
			TokenValue::Greater => BinaryOperator::Greater,
			TokenValue::GreaterEqual => BinaryOperator::GreaterEqual,

			_ => return None,
		};

		self.pointer += 1;

		Some(Node {
			span: token.span.clone(),
			val: op,
		})
	}

	fn token_next_add_op(&mut self) -> Option<Node<BinaryOperator>> {
		let token = self.tokens.get(self.pointer)?;

		let op = match token.value {
			TokenValue::Plus => BinaryOperator::Add,
			TokenValue::Minus => BinaryOperator::Subtract,

			_ => return None,
		};

		self.pointer += 1;

		Some(Node {
			span: token.span.clone(),
			val: op,
		})
	}

	fn token_next_mul_op(&mut self) -> Option<Node<BinaryOperator>> {
		let token = self.tokens.get(self.pointer)?;

		let op = match token.value {
			TokenValue::Star => BinaryOperator::Multiply,
			TokenValue::Slash => BinaryOperator::Divide,
			TokenValue::Percent => BinaryOperator::Modulo,

			_ => return None,
		};

		self.pointer += 1;

		Some(Node {
			span: token.span.clone(),
			val: op,
		})
	}

	fn token_next_unary_op(&mut self) -> Option<Node<UnaryOperator>> {
		let token = self.tokens.get(self.pointer)?;

		let op = match token.value {
			TokenValue::Minus => UnaryOperator::Negate,
			TokenValue::Bang => UnaryOperator::Flip,

			_ => return None,
		};

		self.pointer += 1;

		Some(Node {
			span: token.span.clone(),
			val: op,
		})
	}

	fn token_rewind(&mut self) {
		if self.pointer > 0 {
			self.pointer -= 1;
		}
	}

	// statements
	fn parse_statement(&mut self) -> Result<Option<Statement>, ParserError> {
		let Some(token) = self.token_peek() else {
			return Ok(None);
		};

		Ok(Some(match token.value {
			TokenValue::EoF => return Ok(None),

			// statements
			TokenValue::Use => Statement::Import(self.parse_use()?),
			TokenValue::Global | TokenValue::At => {
				Statement::GlobalDeclaration(self.parse_global()?)
			}
			TokenValue::Order => Statement::Order(self.parse_order()?),
			TokenValue::Fn => Statement::FunctionDeclaration(self.parse_fn()?),
			TokenValue::Let => Statement::Let(self.parse_let()?),
			TokenValue::Par => Statement::Par(self.parse_par()?),
			TokenValue::Sleep => Statement::Sleep(self.parse_sleep()?),
			TokenValue::Print => Statement::Print(self.parse_print()?),
			TokenValue::If => Statement::If(self.parse_if()?),
			TokenValue::While => Statement::While(self.parse_while()?),
			TokenValue::Continue => Statement::Continue(self.parse_continue()?),
			TokenValue::Break => Statement::Break(self.parse_break()?),
			TokenValue::Return => Statement::Return(self.parse_return()?),

			// expression
			_ => Statement::Expression(self.parse_expression()?),
		}))
	}

	fn parse_use(&mut self) -> Result<Node<Import>, ParserError> {
		let start = self.token_expect(TokenValue::Use)?.span.start.clone();
		todo!()
	}

	fn parse_global(&mut self) -> Result<Node<GlobalDeclaration>, ParserError> {
		let start = self.token_expect(TokenValue::Global)?.span.start.clone();

		let Some(id_token) = self.token_next().cloned() else {
			return Err(ParserError::UnexpectedEoF);
		};
		let TokenValue::Identifier(id) = &id_token.value else {
			return Err(ParserError::UnexpectedToken {
				src: id_token.src.clone(),
				span: id_token.span.clone(),
			});
		};

		self.token_expect(TokenValue::Colon)?;

		let type_ = self.parse_type()?;

		let id_node = Node {
			span: Span {
				start: id_token.span.start.clone(),
				end: type_.span.end.clone(),
			},
			val: IdentifierTyped {
				id: Node {
					span: id_token.span.clone(),
					val: Identifier(id.to_owned()),
				},
				type_,
			},
		};

		self.token_expect(TokenValue::Equal)?;

		// TODO: make it optional
		let value = self.parse_expression()?;

		// TODO: parse global description

		let end = self.token_expect(TokenValue::Semicolon)?.span.end.clone();

		Ok(Node {
			span: Span { start, end },
			val: GlobalDeclaration {
				id: id_node,
				value: Some(value),
				description: None,
			},
		})
	}

	fn parse_order(&mut self) -> Result<Node<Order>, ParserError> {
		let start = self.token_expect(TokenValue::Order)?.span.start.clone();
		self.in_order += 1;
		let block = self.parse_block()?;
		self.in_order -= 1;

		Ok(Node {
			span: Span {
				start,
				end: block.span.end.clone(),
			},
			val: Order { block },
		})
	}

	fn parse_fn(&mut self) -> Result<Node<FunctionDeclaration>, ParserError> {
		let start = self.token_expect(TokenValue::Fn)?.span.start.clone();
		self.in_function += 1;
		// parse
		self.in_function -= 1;
		todo!()
	}

	fn parse_let(&mut self) -> Result<Node<Let>, ParserError> {
		let start = self.token_expect(TokenValue::Let)?.span.start.clone();

		let Some(id_token) = self.token_next().cloned() else {
			return Err(ParserError::UnexpectedEoF);
		};
		let TokenValue::Identifier(id) = &id_token.value else {
			return Err(ParserError::UnexpectedToken {
				src: id_token.src.clone(),
				span: id_token.span.clone(),
			});
		};

		self.token_expect(TokenValue::Colon)?;

		let type_ = self.parse_type()?;

		let id_node = Node {
			span: Span {
				start: id_token.span.start.clone(),
				end: type_.span.end.clone(),
			},
			val: IdentifierTyped {
				id: Node {
					span: id_token.span.clone(),
					val: Identifier(id.to_owned()),
				},
				type_,
			},
		};

		self.token_expect(TokenValue::Equal)?;

		// TODO: make it optional
		let value = self.parse_expression()?;

		let end = self.token_expect(TokenValue::Semicolon)?.span.end.clone();

		Ok(Node {
			span: Span { start, end },
			val: Let { id: id_node, value },
		})
	}

	fn parse_par(&mut self) -> Result<Node<Par>, ParserError> {
		let start = self.token_expect(TokenValue::Par)?.span.start.clone();
		self.token_expect(TokenValue::CurlyOpen)?;
		self.in_par += 1;

		let mut blocks = Vec::new();
		while let Some(peek) = self.token_peek() {
			if peek.value == TokenValue::CurlyOpen {
				blocks.push(self.parse_block()?);
			} else {
				break;
			}
		}

		self.in_par -= 1;
		let end = self.token_expect(TokenValue::CurlyClose)?.span.end.clone();

		Ok(Node {
			span: Span { start, end },
			val: Par { blocks },
		})
	}

	fn parse_sleep(&mut self) -> Result<Node<Sleep>, ParserError> {
		let start = self.token_expect(TokenValue::Sleep)?.span.start.clone();

		let time = self.parse_expression()?;

		let end = self.token_expect(TokenValue::Semicolon)?.span.end.clone();

		Ok(Node {
			span: Span { start, end },
			val: Sleep { time },
		})
	}

	fn parse_print(&mut self) -> Result<Node<Print>, ParserError> {
		let start = self.token_expect(TokenValue::Print)?.span.start.clone();

		let value = self.parse_expression()?;

		let end = self.token_expect(TokenValue::Semicolon)?.span.end.clone();

		Ok(Node {
			span: Span { start, end },
			val: Print { value },
		})
	}

	fn parse_if(&mut self) -> Result<Node<If>, ParserError> {
		let start = self.token_expect(TokenValue::If)?.span.start.clone();

		let condition = self.parse_expression()?;

		let block = self.parse_block()?;

		let mut else_ = None;
		if let Some(token) = self.token_peek() {
			if token.value == TokenValue::Else {
				else_ = Some(Box::new(self.parse_else()?));
			}
		}

		Ok(Node {
			span: Span {
				start,
				end: block.span.end.clone(),
			},
			val: If {
				condition,
				then: block,
				else_,
			},
		})
	}

	fn parse_else(&mut self) -> Result<Node<Else>, ParserError> {
		let start = self.token_expect(TokenValue::Else)?.span.start.clone();

		let Some(peek) = self.token_peek() else {
			return Err(ParserError::UnexpectedEoF);
		};

		if peek.value == TokenValue::CurlyOpen {
			let block = self.parse_block()?;
			Ok(Node {
				span: Span {
					start,
					end: block.span.end.clone(),
				},
				val: Else::Else(block),
			})
		} else {
			let if_ = self.parse_if()?;

			Ok(Node {
				span: Span {
					start,
					end: if_.span.end.clone(),
				},
				val: Else::ElseIf(if_),
			})
		}
	}

	fn parse_while(&mut self) -> Result<Node<While>, ParserError> {
		let start = self.token_expect(TokenValue::While)?.span.start.clone();

		let condition = self.parse_expression()?;

		self.in_loop += 1;
		let block = self.parse_block()?;
		self.in_loop -= 1;

		Ok(Node {
			span: Span {
				start,
				end: block.span.end.clone(),
			},
			val: While {
				condition,
				do_: block,
			},
		})
	}

	fn parse_continue(&mut self) -> Result<Node<Continue>, ParserError> {
		let start = self.token_expect(TokenValue::Continue)?.span.start.clone();

		let end = self.token_expect(TokenValue::Semicolon)?.span.end.clone();

		Ok(Node {
			span: Span { start, end },
			val: Continue,
		})
	}

	fn parse_break(&mut self) -> Result<Node<Break>, ParserError> {
		let start = self.token_expect(TokenValue::Break)?.span.start.clone();

		let end = self.token_expect(TokenValue::Semicolon)?.span.end.clone();

		Ok(Node {
			span: Span { start, end },
			val: Break,
		})
	}

	fn parse_return(&mut self) -> Result<Node<Return>, ParserError> {
		let start = self.token_expect(TokenValue::Return)?.span.start.clone();

		let value = self.parse_expression()?;

		let end = self.token_expect(TokenValue::Semicolon)?.span.end.clone();

		Ok(Node {
			span: Span { start, end },
			val: Return { value },
		})
	}

	// statements helper
	fn parse_type(&mut self) -> Result<Node<Type>, ParserError> {
		todo!()
	}

	fn parse_block(&mut self) -> Result<Node<Block>, ParserError> {
		let start = self.token_expect(TokenValue::CurlyOpen)?.span.start.clone();

		let mut stmts = Vec::new();
		loop {
			if let Some(Token {
				value: TokenValue::CurlyClose,
				..
			}) = self.token_peek()
			{
				break;
			}

			let Some(stmt) = self.parse_statement()? else {
				return Err(ParserError::UnexpectedEoF);
			};

			// TODO: add checks: break only in loops, etc

			stmts.push(stmt);
		}

		let end = self.token_expect(TokenValue::CurlyClose)?.span.end.clone();

		Ok(Node {
			span: Span { start, end },
			val: Block { stmts },
		})
	}

	// expressions
	fn parse_expression(&mut self) -> Result<Expression, ParserError> {
		self.parse_assignment()
	}

	fn parse_assignment(&mut self) -> Result<Expression, ParserError> {
		let expr = self.parse_or()?;

		let Some(peek) = self.token_peek() else {
			return Ok(expr);
		};
		if peek.value != TokenValue::Equal {
			return Ok(expr);
		}

		let Expression::IdentifierFull(id) = expr else {
			return Ok(expr);
		};

		self.token_expect(TokenValue::Equal)?;

		let value = self.parse_or()?;

		Ok(Expression::Assignment(Node {
			span: Span {
				start: id.span.start.clone(),
				end: value.get_span().end.clone(),
			},
			val: Assignment {
				id: id,
				value: Box::new(value),
			},
		}))
	}

	fn parse_or(&mut self) -> Result<Expression, ParserError> {
		let mut left = self.parse_and()?;

		while let Some(op) = self.token_want(TokenValue::LOr).cloned() {
			let right = self.parse_and()?;

			left = Expression::Logical(Node {
				span: Span {
					start: left.get_span().start.clone(),
					end: right.get_span().end.clone(),
				},
				val: Logical {
					left: Box::new(left),
					op: Node {
						span: op.span,
						val: LogicalOperator::Or,
					},
					right: Box::new(right),
				},
			})
		}

		Ok(left)
	}

	fn parse_and(&mut self) -> Result<Expression, ParserError> {
		let mut left = self.parse_comparison()?;

		while let Some(op) = self.token_want(TokenValue::LOr).cloned() {
			let right = self.parse_comparison()?;

			left = Expression::Logical(Node {
				span: Span {
					start: left.get_span().start.clone(),
					end: right.get_span().end.clone(),
				},
				val: Logical {
					left: Box::new(left),
					op: Node {
						span: op.span,
						val: LogicalOperator::Or,
					},
					right: Box::new(right),
				},
			})
		}

		Ok(left)
	}

	fn parse_comparison(&mut self) -> Result<Expression, ParserError> {
		let mut left = self.parse_additive()?;

		while let Some(op) = self.token_next_comp_op() {
			let right = self.parse_additive()?;

			left = Expression::Binary(Node {
				span: Span {
					start: left.get_span().start.clone(),
					end: right.get_span().end.clone(),
				},
				val: Binary {
					left: Box::new(left),
					op,
					right: Box::new(right),
				},
			})
		}

		Ok(left)
	}

	fn parse_additive(&mut self) -> Result<Expression, ParserError> {
		let mut left = self.parse_multiplicative()?;

		while let Some(op) = self.token_next_add_op() {
			let right = self.parse_multiplicative()?;

			left = Expression::Binary(Node {
				span: Span {
					start: left.get_span().start.clone(),
					end: right.get_span().end.clone(),
				},
				val: Binary {
					left: Box::new(left),
					op,
					right: Box::new(right),
				},
			})
		}

		Ok(left)
	}

	fn parse_multiplicative(&mut self) -> Result<Expression, ParserError> {
		let mut left = self.parse_unary()?;

		while let Some(op) = self.token_next_mul_op() {
			let right = self.parse_unary()?;

			left = Expression::Binary(Node {
				span: Span {
					start: left.get_span().start.clone(),
					end: right.get_span().end.clone(),
				},
				val: Binary {
					left: Box::new(left),
					op,
					right: Box::new(right),
				},
			})
		}

		Ok(left)
	}

	fn parse_unary(&mut self) -> Result<Expression, ParserError> {
		if let Some(op) = self.token_next_unary_op() {
			let value = self.parse_unary()?;

			Ok(Expression::Unary(Node {
				span: Span {
					start: op.span.start.clone(),
					end: value.get_span().end.clone(),
				},
				val: Unary {
					op,
					right: Box::new(value),
				},
			}))
		} else {
			self.parse_atomic()
		}
	}

	/*fn parse_member_call_index(&mut self) -> Result<Expression, ParserError> {
		todo!()
	}*/

	fn parse_atomic(&mut self) -> Result<Expression, ParserError> {
		let Some(token) = self.token_next() else {
			return Err(ParserError::UnexpectedEoF);
		};

		let expr = match &token.value {
			TokenValue::Null => Expression::Literal(Node {
				span: token.span.clone(),
				val: Literal::Null,
			}),
			TokenValue::Bool(b) => Expression::Literal(Node {
				span: token.span.clone(),
				val: Literal::Bool(*b),
			}),
			TokenValue::Number(n) => Expression::Literal(Node {
				span: token.span.clone(),
				val: Literal::Number(*n),
			}),
			TokenValue::String(s) => Expression::Literal(Node {
				span: token.span.clone(),
				val: Literal::String(s.to_owned()),
			}),
			TokenValue::ParenOpen => {
				let expr = self.parse_expression()?;
				self.token_expect(TokenValue::ParenClose)?;
				expr
			}
			TokenValue::Identifier(id) => {
				// TODO: fix for ksdf::ksdjfk
				Expression::IdentifierFull(Node {
					span: token.span.clone(),
					val: IdentifierFull {
						id: Node {
							span: token.span.clone(),
							val: Identifier(id.clone()),
						},
						module: Vec::new(),
					},
				})
			}
			// TODO: arrays, objects
			_ => {
				return Err(ParserError::UnexpectedToken {
					src: token.src.clone(),
					span: token.span.clone(),
				})
			}
		};

		Ok(expr)
	}
}
