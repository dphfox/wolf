use std::iter::Peekable;

use serde::Serialize;
use wf_token::{Span, Token, TokenType};

pub mod explain;

#[derive(Debug, Clone, Serialize)]
pub enum BiOp { Exponent, Multiply, Divide, FloorDivide, CeilDivide, FloorMod, Plus, Minus, Equal, NotEqual, Less, More, LessEqual, MoreEqual, And, Or }

#[derive(Debug, Clone, Serialize)]
pub enum UnOp { Not, Negate, DoubleNegate, Count }

#[derive(Debug, Clone, Serialize)]
pub enum Parse {
	Block { lets: Vec<Parse>, expr: Box<Parse> },
	Let { capture: Box<Parse>, expr: Box<Parse> },

	ExprThrow { expr: Box<Parse> },
	ExprChain { first: Box<Parse>, rest: Vec<Parse> },
	ExprAutoChainFirstBiOp { bi_op: BiOp, operand: Box<Parse> },
	ExprAutoChainFirstFnEval { name: String, datum: Option<Box<Parse>> },
	ExprInfix { first: Box<Parse>, rest: Vec<(BiOp, Parse)> },
	ExprPrefix { un_ops: Vec<UnOp>, term: Box<Parse> },
	ExprAccess { accesses: Vec<String>, term: Box<Parse> },

	ValueFnEval { name: String, datum: Box<Parse> },
	ValueName { name: String },
	ValueConditional { if_expr: Box<Parse>, then_expr: Box<Parse>, else_expr: Box<Parse> },
	ValueLoop { capture: Box<Parse>, initial_expr: Box<Parse>, body: Box<Parse> },
	ValueBlock { catch: bool, block: Box<Parse> },
	ValueFnDef { capture: Box<Parse>, expr: Box<Parse> },
	ValueString { string: String },
	ValueTuple { entries: Vec<Parse> },
	ValueTupleEntry { matcher: Option<Token>, value: Box<Parse> },

	CaptureName { name: String },
	CaptureTuple { entries: Vec<Parse> },
	CaptureTupleEntry { matcher: Option<Token>, capture: Option<Box<Parse>>, ty: Option<Box<Parse>> },

	TypeName { name: String }
}

#[derive(Debug, Clone, Serialize)]
pub enum ErrorInParse {
	UnexpectedToken { token: Token, expected: &'static str },
	UnexpectedEndOfFile { expected: &'static str },
	NotYetImplemented { note: &'static str },
	Context { start: Option<Span>, name: &'static str, inner: Box<ErrorInParse> }
}

// FUTURE: use try {} block for this instead
macro_rules! err_context {
	($self:expr, $name:expr, $block:block) => {{
		let span = match $self.tokens.peek() {
			Some(tok) => Some(tok.span.clone()),
			None => None
		};
		(move || Ok($block))().map_err(move |e| match e {
			ErrorInParse::Context { start, .. } if start == span => e,
			e => ErrorInParse::Context {
				start: span,
				name: $name, 
				inner: Box::new(e)
			}
		})
	}};
}

macro_rules! gap {
	($self:expr, unstoppable) => {{
		loop {
			let Some(token) = $self.tokens.peek() else { break None };
			match token.ty {
				TokenType::Whitespace | TokenType::EndLine | TokenType::Comment { .. } => { let _ = $self.tokens.next(); },
				_ => { break Some(token); },
			}
		}
	}};
	($self:expr, stop_at_line) => {{
		loop {
			let Some(token) = $self.tokens.peek() else { break None };
			match token.ty {
				TokenType::Whitespace | TokenType::Comment { .. } => { let _ = $self.tokens.next(); },
				_ => { break Some(token); },
			}
		}
	}};
}

macro_rules! is_of_type {
	($self:expr, $ty:ident) => {{
		matches!($self.tokens.peek(), Some(Token { ty: TokenType::$ty { .. }, .. }))
	}};
}

macro_rules! consume {
	($self:expr, already_checked) => {{
		$self.tokens.next().expect("consume should only ever be called after peeking to ensure type")
	}};
	($self:expr, $expect:expr) => {{
		if let Some(token) = $self.tokens.next() {
			Ok(token)
		} else {
			Err(ErrorInParse::UnexpectedEndOfFile { expected: $expect })
		}
	}};
	($self:expr, $ty:ident, $expect:expr) => {{
		if let Some(token) = $self.tokens.next() {
			if matches!(token.ty, TokenType::$ty { .. }){ 
				Ok(token) 
			} else { 
				Err(ErrorInParse::UnexpectedToken { token, expected: $expect })
			}
		} else {
			Err(ErrorInParse::UnexpectedEndOfFile { expected: $expect })
		}
	}};
}

macro_rules! expected {
	($self:expr, $token:expr, $expect:expr) => {{
		if let Some(token) = $token {
			return Err(ErrorInParse::UnexpectedToken { token, expected: $expect })
		} else {
			return Err(ErrorInParse::UnexpectedEndOfFile { expected: $expect })
		}		
	}};
}

pub struct Parser<Input: Iterator<Item = Token>> {
	tokens: Peekable<Input>
}

impl<Input: Iterator<Item = Token>> Parser<Input> {	
	pub fn new(input: Input) -> Self {
        Parser { 
            tokens: input.peekable()
        }
    }

	fn parse_block(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "block", {
			consume!(self, OpenParen, "opening parenthesis of block")?;
			gap!(self, unstoppable);
			let block = self.parse_block_inner()?;
			gap!(self, unstoppable);
			consume!(self, CloseParen, "closing parenthesis of block")?;
			block
		})
	}

	fn parse_block_inner(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "block contents", {
			let mut lets = vec![];
			while self.peek_let_declaration() {
				lets.push(self.parse_let_declaration()?);
				gap!(self, unstoppable);
			}
			let expr = self.parse_expr()?;
			while self.peek_let_declaration() {
				gap!(self, unstoppable);
				lets.push(self.parse_let_declaration()?);
			}
			Parse::Block { lets, expr: Box::new(expr) }
		})
	}

	fn peek_let_declaration(&mut self) -> bool {
		is_of_type!(self, Let)
	}

	fn parse_let_declaration(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "let declaration", {
			consume!(self, Let, "let")?;
			gap!(self, unstoppable);
			let capture = Box::new(self.parse_capture()?);
			gap!(self, unstoppable);
			consume!(self, Equal, "assignment")?;
			gap!(self, unstoppable);
			let expr = self.parse_expr()?;
			Parse::Let { capture, expr: Box::new(expr) }
		})
	}

	fn parse_expr(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "expression", {
			if is_of_type!(self, Throw) {
				consume!(self, already_checked);
				gap!(self, unstoppable);
				Parse::ExprThrow { expr: Box::new(self.parse_expr_chain()?) }
			} else {
				self.parse_expr_chain()?
			}
		})
	}

	fn parse_expr_chain(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "expression chain", {
			let first = self.parse_expr_infix()?;
			gap!(self, stop_at_line);
			if is_of_type!(self, ThinArrow) || is_of_type!(self, FatArrow) {
				let mut rest = vec![];
				loop {
					if is_of_type!(self, ThinArrow) {
						consume!(self, already_checked);
						gap!(self, unstoppable);
						rest.push(self.parse_expr_infix()?);
						gap!(self, stop_at_line);
					} else if is_of_type!(self, FatArrow) {
						consume!(self, already_checked);
						gap!(self, unstoppable);
						rest.push(self.parse_expr_infix_auto()?);
						gap!(self, stop_at_line);
					} else {
						break;
					}
				}
				Parse::ExprChain { first: Box::new(first), rest }
			} else {
				first
			}
		})
	}

	fn parse_expr_infix(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "infix operation", {
			let first = self.parse_expr_prefix()?;
			gap!(self, stop_at_line);
			if self.peek_bi_op().is_some() {
				let mut rest = vec![];
				while let Some(bi_op) = self.peek_bi_op() {
					consume!(self, already_checked);
					gap!(self, unstoppable);
					rest.push((bi_op, self.parse_expr_prefix()?));
					gap!(self, stop_at_line);
				}
				Parse::ExprInfix { first: Box::new(first), rest }
			} else {
				first
			}
		})
	}

	fn parse_expr_infix_auto(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "auto-chained infix operation", {
			let first = if let Some(bi_op) = self.peek_bi_op() {
				consume!(self, already_checked);
				gap!(self, unstoppable);
				Parse::ExprAutoChainFirstBiOp { bi_op, operand: Box::new(self.parse_expr_prefix()?) }
			} else {
				match self.tokens.next() {
					Some(Token { ty: TokenType::Name { name }, .. }) => {
						gap!(self, stop_at_line);
						let datum = if self.peek_value_tuple() { Some(Box::new(self.parse_value_tuple()?)) } else { None };
						Parse::ExprAutoChainFirstFnEval { name, datum }
					},
					token => expected!(self, token, "infix operator or function evaluation")
				}
			};
			gap!(self, stop_at_line);
			let mut rest = vec![];
			while let Some(bi_op) = self.peek_bi_op() {
				consume!(self, already_checked);
				gap!(self, unstoppable);
				rest.push((bi_op, self.parse_expr_prefix()?));
				gap!(self, stop_at_line);
			}
			Parse::ExprInfix { first: Box::new(first), rest }
		})
	}

	fn parse_expr_prefix(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "prefix operation", {
			if self.peek_un_op().is_some() {
				let mut un_ops = vec![];
				while let Some(un_op) = self.peek_un_op() {
					consume!(self, already_checked);
					gap!(self, unstoppable);
					un_ops.push(un_op);
				}
				Parse::ExprPrefix { un_ops, term: Box::new(self.parse_expr_access()?) }
			} else {
				self.parse_expr_access()?
			}
		})
	}

	fn parse_expr_access(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "named access", {
			let term = self.parse_value()?;
			gap!(self, stop_at_line);
			if is_of_type!(self, Dot) {
				let mut accesses = vec![];
				loop {
					consume!(self, already_checked);
					gap!(self, unstoppable);
					match self.tokens.next() {
						Some(Token { ty: TokenType::Name { name }, .. }) => accesses.push(name),
						token => expected!(self, token, "name to be accessed")
					}
					gap!(self, stop_at_line);
					if !is_of_type!(self, Dot) {
						break;
					}
				}
				Parse::ExprAccess { accesses, term: Box::new(term) }
			} else {
				term
			}
		})
	}

	fn peek_bi_op(&mut self) -> Option<BiOp> {
		let token = self.tokens.peek()?;
		let ty = match token.ty {
			TokenType::Caret => BiOp::Exponent,
			TokenType::Asterisk => BiOp::Multiply,
			TokenType::Slash => BiOp::Divide,
			TokenType::DoubleSlash => BiOp::FloorDivide,
			TokenType::SlashCaret => BiOp::CeilDivide,
			TokenType::Percent => BiOp::FloorMod,
			TokenType::Plus => BiOp::Plus,
			TokenType::Minus => BiOp::Minus,
			TokenType::Equal => BiOp::Equal,
			TokenType::BangEqual => BiOp::NotEqual,
			TokenType::Less => BiOp::Less,
			TokenType::More => BiOp::More,
			TokenType::LessEqual => BiOp::LessEqual,
			TokenType::MoreEqual => BiOp::MoreEqual,
			TokenType::And => BiOp::And,
			TokenType::Or => BiOp::Or,
			_ => return None
		};
		Some(ty)
	}
	
	fn peek_un_op(&mut self) -> Option<UnOp> {
		let token = self.tokens.peek()?;
		let ty = match token.ty {
			TokenType::Bang => UnOp::Not,
			TokenType::Minus => UnOp::Negate,
			TokenType::Plus => UnOp::DoubleNegate,
			TokenType::Hash => UnOp::Count,
			_ => return None
		};
		Some(ty)
	}

	fn parse_value(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "value", {
			if self.peek_value_tuple() {
				self.parse_value_tuple()?
			} else if self.peek_value_conditional() {
				self.parse_value_conditional()?
			} else if self.peek_value_loop() {
				self.parse_value_loop()?
			} else if self.peek_value_block() {
				self.parse_value_block()?
			} else if self.peek_value_fn_def() {
				self.parse_value_fn_def()?
			} else {
				match self.tokens.next() {
					Some(Token { ty: TokenType::Name { name }, .. }) => {
						gap!(self, stop_at_line);
						if self.peek_value_tuple() {
							Parse::ValueFnEval { name, datum: Box::new(self.parse_value_tuple()?) }
						} else {
							Parse::ValueName { name }
						}
					},
					Some(Token { ty: TokenType::String { string }, .. }) => Parse::ValueString { string },
					token => expected!(self, token, "function evaluation, name, string, tuple, conditional, loop, block, or function definition")
				}
			}
		})
	}

	fn peek_value_tuple(&mut self) -> bool {
		is_of_type!(self, OpenBracket)
	}

	fn parse_value_tuple(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "tuple value", {
			consume!(self, OpenBracket, "opening bracket of tuple")?;
			gap!(self, unstoppable);
			let mut entries = vec![];
			loop {
				if is_of_type!(self, CloseBracket) {
					consume!(self, already_checked);
					break;
				}
				entries.push(self.parse_value_tuple_entry()?);
				gap!(self, stop_at_line);
				if is_of_type!(self, CloseBracket) {
					consume!(self, already_checked);
					break;
				} else if is_of_type!(self, Comma) || is_of_type!(self, EndLine) {
					consume!(self, already_checked);
					gap!(self, unstoppable);
				} else {
					expected!(self, self.tokens.next(), "closing bracket of tuple, or comma or new line to separate tuple entries");
				}
			}
			Parse::ValueTuple { entries }
		})
	}

	fn parse_value_tuple_entry(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "tuple value entry", {
			let matcher = if is_of_type!(self, Ellipsis) {
				let ellipsis = consume!(self, already_checked);
				Some(ellipsis)
			} else if is_of_type!(self, Dot) {
				consume!(self, already_checked);
				gap!(self, unstoppable);
				let name = consume!(self, Name, "name for tuple entry")?;
				Some(name)
			} else { None };
			gap!(self, unstoppable);
			let value = Box::new(self.parse_expr()?);
			Parse::ValueTupleEntry { matcher, value }
		})
	}

	fn peek_value_conditional(&mut self) -> bool {
		is_of_type!(self, If)
	}

	fn parse_value_conditional(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "conditional value", {
			consume!(self, If, "if")?;
			gap!(self, unstoppable);
			let if_expr = Box::new(self.parse_expr()?);
			gap!(self, unstoppable);
			consume!(self, Then, "then")?;
			gap!(self, unstoppable);
			let then_expr = Box::new(self.parse_expr()?);
			gap!(self, unstoppable);
			consume!(self, Else, "else")?;
			gap!(self, unstoppable);
			let else_expr = Box::new(self.parse_expr()?);
			Parse::ValueConditional { if_expr, then_expr, else_expr }
		})
	}

	fn peek_value_loop(&mut self) -> bool {
		is_of_type!(self, Loop)
	}

	fn parse_value_loop(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "loop value", {
			consume!(self, Loop, "loop")?;
			gap!(self, unstoppable);
			let capture = Box::new(self.parse_capture()?);
			gap!(self, unstoppable);
			consume!(self, Equal, "initial assignment")?;
			gap!(self, unstoppable);
			let initial_expr = Box::new(self.parse_expr()?);
			gap!(self, unstoppable);
			let body = Box::new(self.parse_block()?);
			Parse::ValueLoop { capture, initial_expr, body }
		})
	}

	fn peek_value_block(&mut self) -> bool {
		is_of_type!(self, Catch) || is_of_type!(self, OpenParen)
	}

	fn parse_value_block(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "block value", {
			let catch = if is_of_type!(self, Catch) {
				consume!(self, already_checked);
				gap!(self, unstoppable);
				true
			} else {
				false
			};
			let block = Box::new(self.parse_block()?);
			Parse::ValueBlock { catch, block }
		})
	}

	fn peek_value_fn_def(&mut self) -> bool {
		is_of_type!(self, Fn)
	}

	fn parse_value_fn_def(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "function definition", {
			consume!(self, Fn, "fn")?;
			gap!(self, unstoppable);
			let capture = Box::new(self.parse_capture_tuple()?);
			gap!(self, unstoppable);
			let expr = Box::new(self.parse_expr()?);
			Parse::ValueFnDef { capture, expr }
		})
	}

	fn parse_capture(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "capture", {
			if self.peek_capture_tuple() {
				self.parse_capture_tuple()?
			} else {
				match self.tokens.next() {
					Some(Token { ty: TokenType::Name { name }, .. }) => Parse::CaptureName { name },
					token => expected!(self, token, "name capture or tuple capture")
				}
			}
		})
	}

	fn peek_capture_tuple(&mut self) -> bool {
		is_of_type!(self, OpenBracket)
	}

	fn parse_capture_tuple(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "tuple capture", {
			consume!(self, OpenBracket, "opening bracket of tuple capture")?;
			gap!(self, unstoppable);
			let mut entries = vec![];
			loop {
				if is_of_type!(self, CloseBracket) {
					consume!(self, already_checked);
					break;
				}
				entries.push(self.parse_capture_tuple_entry()?);
				gap!(self, stop_at_line);
				if is_of_type!(self, CloseBracket) {
					consume!(self, already_checked);
					break;
				} else if is_of_type!(self, Comma) || is_of_type!(self, EndLine) {
					consume!(self, already_checked);
					gap!(self, unstoppable);
				} else {
					expected!(self, self.tokens.next(), "closing bracket of tuple, or comma or new line to separate tuple entries");
				}
			}
			Parse::CaptureTuple { entries }
		})
	}

	fn parse_capture_tuple_entry(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "tuple capture entry", {
			let (matcher, capture) = if is_of_type!(self, Ellipsis) {
				let ellipsis = consume!(self, already_checked);
				gap!(self, unstoppable);
				let capture = Box::new(self.parse_capture()?);
				gap!(self, stop_at_line);
				(Some(ellipsis), Some(capture))
			} else if is_of_type!(self, Dot) {
				consume!(self, already_checked);
				gap!(self, unstoppable);
				let name = consume!(self, Name, "name to access for tuple capture")?;
				gap!(self, stop_at_line);
				if is_of_type!(self, Comma) || is_of_type!(self, EndLine) || is_of_type!(self, CloseBracket) || is_of_type!(self, Colon) {
					(Some(name), None)
				} else {
					let capture = Box::new(self.parse_capture()?);
					gap!(self, stop_at_line);
					(Some(name), Some(capture))
				}
			} else {
				let capture = Box::new(self.parse_capture()?);
				gap!(self, stop_at_line);
				(None, Some(capture))
			};
			let ty = if is_of_type!(self, Colon) {
				consume!(self, already_checked);
				gap!(self, unstoppable);
				Some(Box::new(self.parse_type()?))
			} else { None };
			Parse::CaptureTupleEntry { matcher, capture, ty }
		})
	}

	fn parse_type(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "type", {
			match self.tokens.next() {
				Some(Token { ty: TokenType::Name { name }, .. }) => Parse::TypeName { name },
				token => expected!(self, token, "capture type name")
			}
		})
	}
}

impl<Input: Iterator<Item = Token>> Iterator for Parser<Input> {
	type Item = Result<Parse, ErrorInParse>;

	fn next(&mut self) -> Option<Self::Item> {
		gap!(self, unstoppable);
		self.tokens.peek()?;
		Some(self.parse_let_declaration())
	}
}