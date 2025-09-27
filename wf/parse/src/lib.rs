// Lossless tokeniser.
// 
// The tokeniser treats all source material as ASCII - UTF-8 characters are not
// specifically part of any syntax, but could still be included in spans like
// names, comments or strings.
// 
// The implementation is intentionally simple for maintainability and for high
// performance. The cost of this is that a whole file is tokenised at once.

use serde::Serialize;
use wf_token::{Token, TokenType};
use wf_lookahead::Lookahead;

#[derive(Debug, Clone, Serialize)]
pub struct ParseInnerBlock {
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
	pub value: Token
}

#[derive(Debug, Clone, Serialize)]
pub struct ParseCapture {
	pub name: Token
}

#[derive(Debug, Clone, Serialize)]
pub enum ParseError {
	NotImplementedYet,
	UnexpectedToken { token: Token, expected: &'static str },
	UnexpectedEndOfFile,

	ErrorInBlockInner(Box<ParseError>),
	ErrorInLetDeclaration(Box<ParseError>),
	ErrorInCapture(Box<ParseError>)
}

trait ParseErrorExt {
	fn in_block_inner(self) -> Self;
	fn in_let_declaration(self) -> Self;
	fn in_capture(self) -> Self;
}

impl<T> ParseErrorExt for Result<T, ParseError> {
	fn in_block_inner(self) -> Self {
		self.map_err(|e| ParseError::ErrorInBlockInner(Box::new(e)))
	}
	fn in_let_declaration(self) -> Self {
		self.map_err(|e| ParseError::ErrorInLetDeclaration(Box::new(e)))
	}
	fn in_capture(self) -> Self {
		self.map_err(|e| ParseError::ErrorInCapture(Box::new(e)))
	}
}

macro_rules! next {
	($self:expr, $pos:expr, across_lines) => {{
		loop {
			let Some(token) = $self.tokens.peek($pos) else { break None };
			match token.ty {
				TokenType::Whitespace | TokenType::EndLine | TokenType::Comment { .. } => { let _ = $self.tokens.consume().next(); },
				_ => { break Some(token); },
			}
		}
	}}; 
}

macro_rules! next_typed {
	($self:expr, $ty:pat, $pos:expr, across_lines) => {{
		next!($self, $pos, across_lines).filter(|tok| matches!(tok.ty, $ty))
	}};
}

macro_rules! literally {
	($self:expr, $ty:ident, $expect:expr, across_lines) => {{
		next!($self, 0, across_lines);
		if let Some(token) = $self.tokens.consume().next() {
			if matches!(token.ty, TokenType::$ty { .. }){ 
				Ok(token) 
			} else { 
				Err(ParseError::UnexpectedToken { token, expected: $expect})
			}
		} else {
			Err(ParseError::UnexpectedEndOfFile)
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

	// block_inner := { let_declaration }, expr, { let_declaration }
	fn parse_block_inner(&mut self) -> Result<ParseInnerBlock, ParseError> {
		let mut let_declarations = vec![];
		while next_typed!(self, TokenType::Let, 0, across_lines).is_some() {
			let_declarations.push(self.parse_let_declaration().in_block_inner()?);
		}
		let expr = self.parse_expr().in_block_inner()?;
		while next_typed!(self, TokenType::Let, 0, across_lines).is_some() {
			let_declarations.push(self.parse_let_declaration().in_block_inner()?);
		}
		Ok(ParseInnerBlock { let_declarations, expr })
	}

	// let_declaration := Let, capture, Equal, expr
	fn parse_let_declaration(&mut self) -> Result<ParseLetDeclaration, ParseError> {
		literally!(self, Let, "let", across_lines).in_let_declaration()?;
		let capture = self.parse_capture().in_let_declaration()?;
		literally!(self, Equal, "assignment", across_lines).in_let_declaration()?;
		let expr = self.parse_expr().in_let_declaration()?;
		Ok(ParseLetDeclaration { capture, expr })
	}

	// TODO: proper expression parsing
	// expr := Name
	fn parse_expr(&mut self) -> Result<ParseExpr, ParseError> {
		let value = literally!(self, Name, "value", across_lines).in_capture()?;
		Ok(ParseExpr { value })
	}

	// TODO: proper capture parsing
	// capture := Name
	fn parse_capture(&mut self) -> Result<ParseCapture, ParseError> {
		let name = literally!(self, Name, "capture", across_lines).in_capture()?;
		Ok(ParseCapture { name })
	}
}

impl<Input: Iterator<Item = Token>> Iterator for Parser<Input> {
	type Item = Result<ParseInnerBlock, ParseError>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.tokens.at_end() { None } else { Some(self.parse_block_inner()) }
	}
}