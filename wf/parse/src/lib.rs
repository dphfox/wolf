// Parse tree generator. Glues together tokens into structures according to the
// grammar of the language, as literally as possible.
//
// Does not handle higher level concepts like desugaring or precedence.

use std::iter::Peekable;

use serde::Serialize;
use wf_token::{Span, Token, TokenType};

pub mod explain;

#[derive(Debug, Clone, Serialize)]
pub struct ParseBlock {
	pub let_declarations: Vec<ParseLetDeclaration>,
	pub expr: Parse
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseLetDeclaration {
	pub capture: ParseCapture,
	pub expr: Parse
}

#[derive(Debug, Clone, Serialize)]
pub enum Parse {
	ExprThrow(Box<Parse>),
	ExprChain { first: Box<Parse>, rest: Vec<Parse> },
	ExprAutoChainFirstBiOp { bi_op: BiOp, operand: Box<Parse> },
	ExprAutoChainFirstFnEval { name: Token, datum: Option<ParseValueTuple> },
	ExprInfix { first: Box<Parse>, rest: Vec<(BiOp, Parse)> },
	ExprPrefix { un_ops: Vec<UnOp>, term: Box<Parse> },
	ExprAccess { accesses: Vec<Token>, term: Box<Parse> },

	ValueFnEval(ParseValueFnEval),
	ValueName(String),
	ValueConditional(ParseValueConditional),
	ValueLoop(ParseValueLoop),
	ValueBlock(ParseValueBlock),
	ValueFn(ParseValueFnDef),
	ValueString(String),
	ValueTuple(ParseValueTuple)
}

#[derive(Debug, Clone, Serialize)]
pub enum BiOp {
	Exponent,
	Multiply,
	Divide,
	FloorDivide,
	CeilDivide,
	FloorMod,
	Plus,
	Minus,
	Equal,
	NotEqual,
	Less,
	More,
	LessEqual,
	MoreEqual,
	And,
	Or
}

#[derive(Debug, Clone, Serialize)]
pub enum UnOp {
	Not,
	Negate,
	DoubleNegate,
	Count
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueFnEval {
	pub name: String,
	pub datum: ParseValueTuple,
}

pub type ParseValueTuple = Vec<ParseValueTupleEntry>;

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueTupleEntry {
	pub matcher: Option<Token>,
	pub value: Box<Parse>
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueConditional {
	pub if_expr: Box<Parse>,
	pub then_expr: Box<Parse>,
	pub else_expr: Box<Parse>
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueLoop {
	pub capture: ParseCapture,
	pub initial_expr: Box<Parse>,
	pub body: Box<ParseBlock>
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueBlock {
	pub catch: bool,
	pub block: Box<ParseBlock>
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueFnDef {
	pub capture: ParseCaptureTuple,
	pub expr: Box<Parse>
}

#[derive(Debug, Clone, Serialize)]
pub enum ParseCapture {
	Name(String),
	Tuple(ParseCaptureTuple)
}

pub type ParseCaptureTuple = Vec<ParseCaptureTupleEntry>;

#[derive(Debug, Clone, Serialize)]
pub struct ParseCaptureTupleEntry {
	pub matcher: Option<Token>,
	pub capture: Option<Box<ParseCapture>>,
	pub ty: Option<ParseCaptureType>
}

#[derive(Debug, Clone, Serialize)]
pub enum ParseCaptureType {
	Name(String)
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
	($self:expr) => {{
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
	($self:expr, $expect:expr) => {{
		if let Some(token) = $self.tokens.next() {
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

	fn parse_block(&mut self) -> Result<ParseBlock, ErrorInParse> {
		err_context!(self, "block", {
			consume!(self, OpenParen, "opening parenthesis of block")?;
			gap!(self, unstoppable);
			let block = self.parse_block_inner()?;
			gap!(self, unstoppable);
			consume!(self, CloseParen, "closing parenthesis of block")?;
			block
		})
	}

	fn parse_block_inner(&mut self) -> Result<ParseBlock, ErrorInParse> {
		err_context!(self, "block contents", {
			let mut let_declarations = vec![];
			while self.peek_let_declaration() {
				let_declarations.push(self.parse_let_declaration()?);
				gap!(self, unstoppable);
			}
			let expr = self.parse_expr()?;
			while self.peek_let_declaration() {
				gap!(self, unstoppable);
				let_declarations.push(self.parse_let_declaration()?);
			}
			ParseBlock { let_declarations, expr }
		})
	}

	fn peek_let_declaration(&mut self) -> bool {
		is_of_type!(self, Let)
	}

	fn parse_let_declaration(&mut self) -> Result<ParseLetDeclaration, ErrorInParse> {
		err_context!(self, "let declaration", {
			consume!(self, Let, "let")?;
			gap!(self, unstoppable);
			let capture = self.parse_capture()?;
			gap!(self, unstoppable);
			consume!(self, Equal, "assignment")?;
			gap!(self, unstoppable);
			let expr = self.parse_expr()?;
			ParseLetDeclaration { capture, expr }
		})
	}

	fn parse_expr(&mut self) -> Result<Parse, ErrorInParse> {
		err_context!(self, "expression", {
			if is_of_type!(self, Throw) {
				consume!(self);
				gap!(self, unstoppable);
				Parse::ExprThrow(Box::new(self.parse_expr_chain()?))
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
						consume!(self);
						gap!(self, unstoppable);
						rest.push(self.parse_expr_infix()?);
						gap!(self, stop_at_line);
					} else if is_of_type!(self, FatArrow) {
						consume!(self);
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
					consume!(self);
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
				consume!(self);
				gap!(self, unstoppable);
				Parse::ExprAutoChainFirstBiOp { bi_op, operand: Box::new(self.parse_expr_prefix()?) }
			} else if is_of_type!(self, Name) {
				let name = consume!(self);
				gap!(self, stop_at_line);
				let datum = if self.peek_value_tuple() {
					Some(self.parse_value_tuple()?)
				} else {
					None
				};
				Parse::ExprAutoChainFirstFnEval { name, datum }
			} else {
				expected!(self, "infix operator or function evaluation")
			};
			gap!(self, stop_at_line);
			let mut rest = vec![];
			while let Some(bi_op) = self.peek_bi_op() {
				consume!(self);
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
					consume!(self);
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
					consume!(self);
					gap!(self, unstoppable);
					accesses.push(consume!(self, Name, "name to be accessed")?);
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
				Parse::ValueTuple(self.parse_value_tuple()?)
			} else if self.peek_value_conditional() {
				Parse::ValueConditional(self.parse_value_conditional()?)
			} else if self.peek_value_loop() {
				Parse::ValueLoop(self.parse_value_loop()?)
			} else if self.peek_value_block() {
				Parse::ValueBlock(self.parse_value_block()?)
			} else if self.peek_value_fn_def() {
				Parse::ValueFn(self.parse_value_fn_def()?)
			} else {
				match consume!(self) {
					Token { ty: TokenType::Name { name }, .. } => {
						gap!(self, stop_at_line);
						if self.peek_value_tuple() {
							Parse::ValueFnEval(ParseValueFnEval { name, datum: self.parse_value_tuple()? })
						} else {
							Parse::ValueName(name)
						}
					},
					Token { ty: TokenType::String { string }, .. } => Parse::ValueString(string),
					_ => expected!(self, "function evaluation, name, string, tuple, conditional, loop, block, or function definition")
				}
			}
		})
	}

	fn peek_value_tuple(&mut self) -> bool {
		is_of_type!(self, OpenBracket)
	}

	fn parse_value_tuple(&mut self) -> Result<ParseValueTuple, ErrorInParse> {
		err_context!(self, "tuple value", {
			consume!(self, OpenBracket, "opening bracket of tuple")?;
			gap!(self, unstoppable);
			let mut entries = vec![];
			loop {
				if is_of_type!(self, CloseBracket) {
					consume!(self);
					break;
				}
				entries.push(self.parse_value_tuple_entry()?);
				gap!(self, stop_at_line);
				if is_of_type!(self, CloseBracket) {
					consume!(self);
					break;
				} else if is_of_type!(self, Comma) || is_of_type!(self, EndLine) {
					consume!(self);
					gap!(self, unstoppable);
				} else {
					expected!(self, "closing bracket of tuple, or comma or new line to separate tuple entries");
				}
			}
			entries
		})
	}

	fn parse_value_tuple_entry(&mut self) -> Result<ParseValueTupleEntry, ErrorInParse> {
		err_context!(self, "tuple value entry", {
			let matcher = if is_of_type!(self, Ellipsis) {
				let ellipsis = consume!(self);
				Some(ellipsis)
			} else if is_of_type!(self, Dot) {
				consume!(self);
				gap!(self, unstoppable);
				let name = consume!(self, Name, "name for tuple entry")?;
				Some(name)
			} else { None };
			gap!(self, unstoppable);
			let value = Box::new(self.parse_expr()?);
			ParseValueTupleEntry { matcher, value }
		})
	}

	fn peek_value_conditional(&mut self) -> bool {
		is_of_type!(self, If)
	}

	fn parse_value_conditional(&mut self) -> Result<ParseValueConditional, ErrorInParse> {
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
			ParseValueConditional { if_expr, then_expr, else_expr }
		})
	}

	fn peek_value_loop(&mut self) -> bool {
		is_of_type!(self, Loop)
	}

	fn parse_value_loop(&mut self) -> Result<ParseValueLoop, ErrorInParse> {
		err_context!(self, "loop value", {
			consume!(self, Loop, "loop")?;
			gap!(self, unstoppable);
			let capture = self.parse_capture()?;
			gap!(self, unstoppable);
			consume!(self, Equal, "initial assignment")?;
			gap!(self, unstoppable);
			let initial_expr = Box::new(self.parse_expr()?);
			gap!(self, unstoppable);
			let body = Box::new(self.parse_block()?);
			ParseValueLoop { capture, initial_expr, body }
		})
	}

	fn peek_value_block(&mut self) -> bool {
		is_of_type!(self, Catch) || is_of_type!(self, OpenParen)
	}

	fn parse_value_block(&mut self) -> Result<ParseValueBlock, ErrorInParse> {
		err_context!(self, "block value", {
			let catch = if is_of_type!(self, Catch) {
				consume!(self);
				gap!(self, unstoppable);
				true
			} else {
				false
			};
			let block = Box::new(self.parse_block()?);
			ParseValueBlock { catch, block }
		})
	}

	fn peek_value_fn_def(&mut self) -> bool {
		is_of_type!(self, Fn)
	}

	fn parse_value_fn_def(&mut self) -> Result<ParseValueFnDef, ErrorInParse> {
		err_context!(self, "function definition", {
			consume!(self, Fn, "fn")?;
			gap!(self, unstoppable);
			let capture = self.parse_capture_tuple()?;
			gap!(self, unstoppable);
			let expr = Box::new(self.parse_expr()?);
			ParseValueFnDef { capture, expr }
		})
	}

	fn parse_capture(&mut self) -> Result<ParseCapture, ErrorInParse> {
		err_context!(self, "capture", {
			if self.peek_capture_tuple() {
				ParseCapture::Tuple(self.parse_capture_tuple()?)
			} else {
				match consume!(self) {
					Token { ty: TokenType::Name { name }, .. } => ParseCapture::Name(name),
					_ => expected!(self, "name capture or tuple capture")
				}
			}
		})
	}

	fn peek_capture_tuple(&mut self) -> bool {
		is_of_type!(self, OpenBracket)
	}

	fn parse_capture_tuple(&mut self) -> Result<ParseCaptureTuple, ErrorInParse> {
		err_context!(self, "tuple capture", {
			consume!(self, OpenBracket, "opening bracket of tuple capture")?;
			gap!(self, unstoppable);
			let mut entries = vec![];
			loop {
				if is_of_type!(self, CloseBracket) {
					consume!(self);
					break;
				}
				entries.push(self.parse_capture_tuple_entry()?);
				gap!(self, stop_at_line);
				if is_of_type!(self, CloseBracket) {
					consume!(self);
					break;
				} else if is_of_type!(self, Comma) || is_of_type!(self, EndLine) {
					consume!(self);
					gap!(self, unstoppable);
				} else {
					expected!(self, "closing bracket of tuple, or comma or new line to separate tuple entries");
				}
			}
			entries
		})
	}

	fn parse_capture_tuple_entry(&mut self) -> Result<ParseCaptureTupleEntry, ErrorInParse> {
		err_context!(self, "tuple capture entry", {
			let (matcher, capture) = if is_of_type!(self, Ellipsis) {
				let ellipsis = consume!(self);
				gap!(self, unstoppable);
				let capture = Box::new(self.parse_capture()?);
				gap!(self, stop_at_line);
				(Some(ellipsis), Some(capture))
			} else if is_of_type!(self, Dot) {
				consume!(self);
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
				consume!(self);
				gap!(self, unstoppable);
				Some(self.parse_capture_type()?)
			} else { None };
			ParseCaptureTupleEntry { matcher, capture, ty }
		})
	}

	fn parse_capture_type(&mut self) -> Result<ParseCaptureType, ErrorInParse> {
		err_context!(self, "capture type", {
			match consume!(self) {
				Token { ty: TokenType::Name { name }, .. } => ParseCaptureType::Name(name),
				_ => expected!(self, "capture type name")
			}
		})
	}
}

impl<Input: Iterator<Item = Token>> Iterator for Parser<Input> {
	type Item = Result<ParseLetDeclaration, ErrorInParse>;

	fn next(&mut self) -> Option<Self::Item> {
		gap!(self, unstoppable);
		self.tokens.peek()?;
		Some(self.parse_let_declaration())
	}
}