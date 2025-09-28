// Parse tree generator. Glues together tokens into structures according to the
// grammar of the language, as literally as possible.
//
// Does not handle higher level concepts like desugaring or precedence.

use serde::Serialize;
use wf_token::{Span, Token, TokenType};
use wf_lookahead::Lookahead;

pub mod explain;

#[derive(Debug, Clone, Serialize)]
pub struct ParseBlock {
	pub let_declarations: Vec<ParseLetDeclaration>,
	pub expr: ParseExpr
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseLetDeclaration {
	pub capture: ParseCapture,
	pub expr: ParseExpr
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseExpr {
	pub throw: bool,
	pub expr_chain: ParseExprChain
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseExprChain {
	pub first: ParseExprInfix,
	pub rest: Vec<ParseExprChainPart>
}

#[derive(Debug, Clone, Serialize)]
pub enum ParseExprChainPart {
	Thin(ParseExprInfix),
	Fat(ParseExprInfixAuto)
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseExprInfix {
	pub first: ParseExprPrefix,
	pub rest: Vec<(ParseBiOp, ParseExprPrefix)>
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseExprInfixAuto {
	pub first: ParseExprInfixAutoFirst,
	pub rest: Vec<(ParseBiOp, ParseExprPrefix)>
}

#[derive(Debug, Clone, Serialize)]
pub enum ParseExprInfixAutoFirst {
	BiOp((ParseBiOp, ParseExprPrefix)),
	FnEval((Token, Option<ParseValueTuple>))
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseExprPrefix {
	pub un_ops: Vec<ParseUnOp>,
	pub term: ParseExprAccess
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseExprAccess {
	pub term: ParseValue,
	pub accesses: Vec<Token>
}

#[derive(Debug, Clone, Serialize)]
pub enum ParseBiOp {
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
pub enum ParseUnOp {
	Not,
	Negate,
	DoubleNegate,
	Count
}

#[derive(Debug, Clone, Serialize)]
pub enum ParseValue {
	FnEval(ParseValueFnEval),
	Name(ParseValueName),
	Conditional(ParseValueConditional),
	Loop(ParseValueLoop),
	Block(ParseValueBlock),
	Fn(ParseValueFnDef),
	String(ParseValueString),
	Tuple(ParseValueTuple)
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueFnEval {
	pub name: Token,
	pub datum: ParseValueTuple,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueName {
	pub name: Token
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueString {
	pub string: Token
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueTuple {
	pub entries: Vec<ParseValueTupleEntry>
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueTupleEntry {
	pub matcher: Option<Token>,
	pub value: Box<ParseExpr>
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueConditional {
	pub if_expr: Box<ParseExpr>,
	pub then_expr: Box<ParseExpr>,
	pub else_expr: Box<ParseExpr>
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseValueLoop {
	pub capture: ParseCapture,
	pub initial_expr: Box<ParseExpr>,
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
	pub expr: Box<ParseExpr>
}

#[derive(Debug, Clone, Serialize)]
pub enum ParseCapture {
	Name(ParseCaptureName),
	Tuple(ParseCaptureTuple)
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseCaptureName {
	pub name: Token
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseCaptureTuple {
	pub entries: Vec<ParseCaptureTupleEntry>
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseCaptureTupleEntry {
	pub matcher: Option<Token>,
	pub capture: Option<Box<ParseCapture>>,
	pub ty: Option<ParseCaptureType>
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseCaptureType {
	pub name: Token
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
		let span = match $self.tokens.peek(0) {
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
			let Some(token) = $self.tokens.peek(0) else { break None };
			match token.ty {
				TokenType::Whitespace | TokenType::EndLine | TokenType::Comment { .. } => { let _ = $self.tokens.consume().next(); },
				_ => { break Some(token); },
			}
		}
	}};
	($self:expr, stop_at_line) => {{
		loop {
			let Some(token) = $self.tokens.peek(0) else { break None };
			match token.ty {
				TokenType::Whitespace | TokenType::Comment { .. } => { let _ = $self.tokens.consume().next(); },
				_ => { break Some(token); },
			}
		}
	}};
}

macro_rules! is_of_type {
	($self:expr, $ty:ident, $pos:expr) => {{
		matches!($self.tokens.peek($pos), Some(Token { ty: TokenType::$ty { .. }, .. }))
	}};
	($self:expr, $ty:ident) => {{
		is_of_type!($self, $ty, 0)
	}};
}

macro_rules! consume {
	($self:expr) => {{
		$self.tokens.consume().next().expect("consume should only ever be called after peeking to ensure type")
	}};
	($self:expr, $expect:expr) => {{
		if let Some(token) = $self.tokens.consume().next() {
			Ok(token)
		} else {
			Err(ErrorInParse::UnexpectedEndOfFile { expected: $expect })
		}
	}};
	($self:expr, $ty:ident, $expect:expr) => {{
		if let Some(token) = $self.tokens.consume().next() {
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
		if let Some(token) = $self.tokens.consume().next() {
			return Err(ErrorInParse::UnexpectedToken { token, expected: $expect })
		} else {
			return Err(ErrorInParse::UnexpectedEndOfFile { expected: $expect })
		}		
	}};
}

pub struct Parser<Input: Iterator<Item = Token>> {
	tokens: Lookahead<8, Token, Input>
}

impl<Input: Iterator<Item = Token>> Parser<Input> {	
	pub fn new(input: Input) -> Self {
		Self { tokens: Lookahead::new(input) }
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

	fn parse_expr(&mut self) -> Result<ParseExpr, ErrorInParse> {
		err_context!(self, "expression", {
			let throw = is_of_type!(self, Throw);
			if throw {
				consume!(self);
				gap!(self, unstoppable);
			}
			let expr_chain = self.parse_expr_chain()?;
			ParseExpr { throw, expr_chain }
		})
	}

	fn parse_expr_chain(&mut self) -> Result<ParseExprChain, ErrorInParse> {
		err_context!(self, "expression chain", {
			let first = self.parse_expr_infix()?;
			gap!(self, stop_at_line);
			let mut rest = vec![];
			loop {
				if is_of_type!(self, ThinArrow) {
					consume!(self);
					gap!(self, unstoppable);
					rest.push(ParseExprChainPart::Thin(self.parse_expr_infix()?));
					gap!(self, stop_at_line);
				} else if is_of_type!(self, FatArrow) {
					consume!(self);
					gap!(self, unstoppable);
					rest.push(ParseExprChainPart::Fat(self.parse_expr_infix_auto()?));
					gap!(self, stop_at_line);
				} else {
					break;
				}
			}
			ParseExprChain { first, rest }
		})
	}

	fn parse_expr_infix(&mut self) -> Result<ParseExprInfix, ErrorInParse> {
		err_context!(self, "infix operation", {
			let first = self.parse_expr_prefix()?;
			gap!(self, stop_at_line);
			let mut rest = vec![];
			while let Some(bi_op) = self.peek_bi_op() {
				consume!(self);
				gap!(self, unstoppable);
				rest.push((bi_op, self.parse_expr_prefix()?));
				gap!(self, stop_at_line);
			}
			ParseExprInfix { first, rest }
		})
	}

	fn parse_expr_infix_auto(&mut self) -> Result<ParseExprInfixAuto, ErrorInParse> {
		err_context!(self, "auto-chained infix operation", {
			let first = if let Some(bi_op) = self.peek_bi_op() {
				consume!(self);
				gap!(self, unstoppable);
				ParseExprInfixAutoFirst::BiOp((bi_op, self.parse_expr_prefix()?))
			} else if is_of_type!(self, Name) {
				let tok = consume!(self);
				gap!(self, stop_at_line);
				let tuple = if self.peek_value_tuple() {
					Some(self.parse_value_tuple()?)
				} else {
					None
				};
				ParseExprInfixAutoFirst::FnEval((tok, tuple))
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
			ParseExprInfixAuto { first, rest }
		})
	}

	fn parse_expr_prefix(&mut self) -> Result<ParseExprPrefix, ErrorInParse> {
		err_context!(self, "prefix operation", {
			let mut un_ops = vec![];
			while let Some(un_op) = self.peek_un_op() {
				consume!(self);
				gap!(self, unstoppable);
				un_ops.push(un_op);
			}
			let term = self.parse_expr_access()?;
			ParseExprPrefix { un_ops, term }
		})
	}

	fn parse_expr_access(&mut self) -> Result<ParseExprAccess, ErrorInParse> {
		err_context!(self, "named access", {
			let term = self.parse_value()?;
			let mut accesses = vec![];
			gap!(self, stop_at_line);
			if is_of_type!(self, Dot) {
				loop {
					consume!(self);
					gap!(self, unstoppable);
					accesses.push(consume!(self, Name, "name to be accessed")?);
					gap!(self, stop_at_line);
					if !is_of_type!(self, Dot) {
						break;
					}
				}
			}
			ParseExprAccess { term, accesses }
		})
	}

	fn peek_bi_op(&mut self) -> Option<ParseBiOp> {
		let token = self.tokens.peek(0)?;
		let ty = match token.ty {
			TokenType::Caret => ParseBiOp::Exponent,
			TokenType::Asterisk => ParseBiOp::Multiply,
			TokenType::Slash => ParseBiOp::Divide,
			TokenType::DoubleSlash => ParseBiOp::FloorDivide,
			TokenType::SlashCaret => ParseBiOp::CeilDivide,
			TokenType::Percent => ParseBiOp::FloorMod,
			TokenType::Plus => ParseBiOp::Plus,
			TokenType::Minus => ParseBiOp::Minus,
			TokenType::Equal => ParseBiOp::Equal,
			TokenType::BangEqual => ParseBiOp::NotEqual,
			TokenType::Less => ParseBiOp::Less,
			TokenType::More => ParseBiOp::More,
			TokenType::LessEqual => ParseBiOp::LessEqual,
			TokenType::MoreEqual => ParseBiOp::MoreEqual,
			TokenType::And => ParseBiOp::And,
			TokenType::Or => ParseBiOp::Or,
			_ => return None
		};
		Some(ty)
	}
	
	fn peek_un_op(&mut self) -> Option<ParseUnOp> {
		let token = self.tokens.peek(0)?;
		let ty = match token.ty {
			TokenType::Bang => ParseUnOp::Not,
			TokenType::Minus => ParseUnOp::Negate,
			TokenType::Plus => ParseUnOp::DoubleNegate,
			TokenType::Hash => ParseUnOp::Count,
			_ => return None
		};
		Some(ty)
	}

	fn parse_value(&mut self) -> Result<ParseValue, ErrorInParse> {
		err_context!(self, "value", {
			if is_of_type!(self, Name) {
				let name = consume!(self);
				gap!(self, stop_at_line);
				if self.peek_value_tuple() {
					ParseValue::FnEval(ParseValueFnEval { name, datum: self.parse_value_tuple()? })
				} else {
					ParseValue::Name(ParseValueName { name })
				}
			} else if is_of_type!(self, String) {
				let string = consume!(self);
				ParseValue::String(ParseValueString { string })
			} else if self.peek_value_tuple() {
				ParseValue::Tuple(self.parse_value_tuple()?)
			} else if self.peek_value_conditional() {
				ParseValue::Conditional(self.parse_value_conditional()?)
			} else if self.peek_value_loop() {
				ParseValue::Loop(self.parse_value_loop()?)
			} else if self.peek_value_block() {
				ParseValue::Block(self.parse_value_block()?)
			} else if self.peek_value_fn_def() {
				ParseValue::Fn(self.parse_value_fn_def()?)
			} else {
				expected!(self, "function evaluation, name, string, tuple, conditional, loop, block, or function definition")
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
			ParseValueTuple { entries }
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
			if is_of_type!(self, Name) {
				let name = consume!(self);
				ParseCapture::Name(ParseCaptureName { name })
			} else if self.peek_capture_tuple() {
				ParseCapture::Tuple(self.parse_capture_tuple()?)
			} else {
				expected!(self, "name capture or tuple capture")
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
			ParseCaptureTuple { entries }
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
			let name = consume!(self, Name, "type name")?;
			gap!(self, stop_at_line);
			ParseCaptureType { name }
		})
	}
}

impl<Input: Iterator<Item = Token>> Iterator for Parser<Input> {
	type Item = Result<ParseLetDeclaration, ErrorInParse>;

	fn next(&mut self) -> Option<Self::Item> {
		gap!(self, unstoppable);
		if self.tokens.at_end() { return None; }
		Some(self.parse_let_declaration())
	}
}