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
pub enum Parse {
	Comment { token: Token }
}

#[derive(Debug, Clone, Serialize)]
pub enum ParseError {
	NotImplementedYet,
	UnexpectedStartToken { token: Token }
}

pub struct Parser<Input: Iterator<Item = Token>> {
	tokens: Lookahead<8, Token, Input>
}

impl<Input: Iterator<Item = Token>> Parser<Input> {
	pub fn new(input: Input) -> Self {
		Self { tokens: Lookahead::new(input) }
	}
}

impl<Input: Iterator<Item = Token>> Iterator for Parser<Input> {
	type Item = Result<Parse, ParseError>;

	fn next(&mut self) -> Option<Self::Item> {
		use TokenType::*;
		let start_position = self.tokens.position();
		macro_rules! ret {
			($ty:expr) => {{
				return Some(Ok($ty));
			}};
		}
		macro_rules! fail {
			($ty:expr) => {{
				if self.tokens.position() == start_position { self.tokens.consume(1); }
				return Some(Err($ty));
			}};
		}

		// TODO: consider a way where we can return consumed things with ownership instead
		let start_token = self.tokens.peek(0)?.clone();

		match start_token.ty {
			// TODO: define a proper grammar for this
			// Start of a throw expression.
			Throw => fail!(ParseError::NotImplementedYet),
			// Start of a catch block.
			Catch => fail!(ParseError::NotImplementedYet),
			// Start of a loop block.
			Loop => fail!(ParseError::NotImplementedYet),
			// Start of a let declaration.
			Let => fail!(ParseError::NotImplementedYet),
			// Start of a function declaration.
			Fn => fail!(ParseError::NotImplementedYet),
			// Start of a conditional.
			If => fail!(ParseError::NotImplementedYet),
			// Start of a tuple literal.
			OpenBracket => fail!(ParseError::NotImplementedYet),
			// Start of a block.
			OpenParen => fail!(ParseError::NotImplementedYet),
			// Start of a unary plus operator.
			Plus => fail!(ParseError::NotImplementedYet),
			// Start of a unary minus operator.
			Minus => fail!(ParseError::NotImplementedYet),
			// Start of a unary boolean not operator.
			Bang => fail!(ParseError::NotImplementedYet),
			// Whitespace.
			Whitespace => fail!(ParseError::NotImplementedYet),
			EndLine => fail!(ParseError::NotImplementedYet),
			// Comment.
			Comment { .. } => {
				self.tokens.consume(1);
				ret!(Parse::Comment { token: start_token })
			},
			// Start of a name literal.
			Name { .. } => fail!(ParseError::NotImplementedYet),
			// Start of a string literal.
			String { .. } => fail!(ParseError::NotImplementedYet),
			
			_ => fail!(ParseError::UnexpectedStartToken { token: start_token })
		}
	}
}