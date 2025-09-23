// Lossless tokeniser.
// 
// The tokeniser treats all source material as ASCII - UTF-8 characters are not
// specifically part of any syntax, but could still be included in spans like
// names, comments or strings.
// 
// The implementation is intentionally simple for maintainability and for high
// performance. The cost of this is that a whole file is tokenised at once.

use serde::Serialize;
use wf_token::{Tokeniser, Token};

#[derive(Debug, Clone, Serialize)]
pub enum Syntax {

}

#[derive(Debug, Clone, Serialize)]
pub enum SyntaxError {
	UnexpectedStartToken(Token)
}

pub struct Parser<'a> {
	tokeniser: Tokeniser<'a>
}

impl<'a> Parser<'a> {
	pub fn new(tokeniser: Tokeniser<'a>) -> Self {
		Self { tokeniser }
	}
}

impl Iterator for Parser<'_> {
	type Item = Result<Syntax, SyntaxError>;

	fn next(&mut self) -> Option<Self::Item> {
		macro_rules! nope {
			($ty:expr) => {
				return Some(Err($ty));
			};
		}

		let start_token = self.tokeniser.next()?;

		nope!(SyntaxError::UnexpectedStartToken(start_token));
	}
}