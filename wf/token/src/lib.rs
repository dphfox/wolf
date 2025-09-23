// Lossless tokeniser.
// 
// The tokeniser treats all source material as ASCII - UTF-8 characters are not
// specifically part of any syntax, but could still be included in spans like
// names, comments or strings.

use std::collections::VecDeque;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Span {
	pub index: usize,
	pub length: usize
}

#[derive(Debug, Clone, Serialize)]
pub struct Token {
	pub ty: TokenType,
	pub span: Span
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TokenType {
	Unexpected,

	Whitespace,
	Comment { num_hyphens: usize }, // 2 = short comment, 3+ = long comment
	Name { backticked: bool },
	String { num_quotes: usize }, // 1 = standard string, 2 = empty string, 3+ = raw string

	Throw,
	Catch,
	Loop,
	Then,
	Else,
	And,
	Let,
	Or,
	Fn,
	If,

	Ellipsis,
	DoubleSlash,
	SlashCaret,
	BangEqual,
	LessEqual,
	MoreEqual,
	ThinArrow,
	FatArrow,
	OpenBracket,
	CloseBracket,
	OpenParen,
	CloseParen,
	Comma,
	Dot,
	Colon,
	Plus,
	Minus,
	Asterisk,
	Slash,
	Caret,
	Equal,
	Bang,
	Less,
	More,
	EndLine,
}

macro_rules! exact {
	($s:expr, $t:ident) => { ($s.as_bytes(), Self::$t) };
}

impl TokenType {
	// Longer strings should come before shorter ones due to greedy matching.
	pub const SORTED_EXACT_TOKENS: &[(&[u8], TokenType)] = &[
		exact!("throw", Throw),
		exact!("catch", Catch),

		exact!("loop", Loop),
		exact!("then", Then),
		exact!("else", Else),

		exact!("and", And),
		exact!("let", Let),
		exact!("...", Ellipsis),

		exact!("or", Or),
		exact!("fn", Fn),
		exact!("if", If),
		exact!("//", DoubleSlash),
		exact!("/^", SlashCaret),
		exact!("!=", BangEqual),
		exact!("<=", LessEqual),
		exact!(">=", MoreEqual),
		exact!("->", ThinArrow),
		exact!("=>", FatArrow),
		exact!("\r\n", EndLine),

		exact!("[", OpenBracket),
		exact!("]", CloseBracket),
		exact!("(", OpenParen),
		exact!(")", CloseParen),
		exact!(",", Comma),
		exact!(".", Dot),
		exact!(":", Colon),
		exact!("+", Plus),
		exact!("-", Minus),
		exact!("*", Asterisk),
		exact!("/", Slash),
		exact!("^", Caret),
		exact!("=", Equal),
		exact!("!", Bang),
		exact!("<", Less),
		exact!(">", More),
		exact!("\n", EndLine),
		exact!("\r", EndLine),
	];

	pub fn external_name(&self) -> &'static str {
		use TokenType::*;
		match self {
			Unexpected => "unexpected",

			Whitespace => "whitespace",
			Comment { .. } => "comment",
			Name { .. } => "name",
			String { .. } => "string",

			Throw => "throw",
			Catch => "catch",
			Loop => "loop",
			Then => "then",
			Else => "else",
			And => "and",
			Let => "let",
			Or => "or",
			Fn => "fn",
			If => "if",

			Ellipsis => "ellipsis",
			DoubleSlash => "double_slash",
			SlashCaret => "slash_caret",
			BangEqual => "bang_equal",
			LessEqual => "less_equal",
			MoreEqual => "more_equal",
			ThinArrow => "thin_arrow",
			FatArrow => "fat_arrow",
			OpenBracket => "open_bracket",
			CloseBracket => "close_bracket",
			OpenParen => "open_paren",
			CloseParen => "close_paren",
			Comma => "comma",
			Dot => "dot",
			Colon => "colon",
			Plus => "plus",
			Minus => "minus",
			Asterisk => "asterisk",
			Slash => "slash",
			Caret => "caret",
			Equal => "equal",
			Bang => "bang",
			Less => "less",
			More => "more",
			EndLine => "end_line",
		}
	}
}

pub struct Tokeniser<Input: Iterator<Item = u8>> {
    input: Input,
    cache: VecDeque<u8>,
	pos: usize
}

impl<Input: Iterator<Item = u8>> Tokeniser<Input> {
    pub fn new(input: Input) -> Self {
        Self {
            input,
            cache: VecDeque::new(),
			pos: 0
        }
    }

    fn ensure_available(&mut self, offset: usize) {
        let remaining_len = (offset + 1).saturating_sub(self.cache.len());
        self.cache.extend(self.input.by_ref().take(remaining_len));
    }

    fn peek(&mut self, offset: usize) -> Option<u8> {
        self.ensure_available(offset);
        self.cache.get(offset).copied()
    }

    fn consume(&mut self, number_of_bytes: usize) -> Option<()> {
		debug_assert_ne!(number_of_bytes, 0);
        self.ensure_available(number_of_bytes - 1);
        if self.cache.len() < number_of_bytes {
            return None;
        }
		self.pos += number_of_bytes;
        for _ in 0..number_of_bytes {
            self.cache.pop_front();
        }
        Some(())
    }
}

impl<Input: Iterator<Item = u8>> Iterator for Tokeniser<Input> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let start_index = self.pos;
		macro_rules! ret {
			($ty:expr) => {
				let length = self.pos - start_index;
				debug_assert!(length > 0, "Zero length tokens aren't valid - they lead to infinite loops");
				return Some(Token { ty: $ty, span: Span { index: start_index, length } });
			};
		}

		// EOF
		let start_char = self.peek(0)?;

		// Comment
		{
			let mut num_hyphens = 0;
			while matches!(self.peek(num_hyphens), Some(b'-')) { num_hyphens += 1; }

			// Long comment
			if num_hyphens > 2 {
				self.consume(num_hyphens);
				let mut end_hyphens = 0;
				while let Some(char) = self.peek(0) {
					self.consume(1);
					if char == b'-' {
						end_hyphens += 1;
						if end_hyphens == num_hyphens { break; }
					} else { end_hyphens = 0; }
				}
				ret!(TokenType::Comment { num_hyphens });
			}

			// Short comment
			if num_hyphens == 2 {
				self.consume(2);
				while let Some(char) = self.peek(0) && !matches!(char, b'\n' | b'\r') { self.consume(1); }
				ret!(TokenType::Comment { num_hyphens });
			}
		}

		// Exact tokens
		for (expect, token) in TokenType::SORTED_EXACT_TOKENS {
			let found_match = expect.iter().enumerate().all(|(offset, &expected)| self.peek(offset) == Some(expected));
			if found_match {
				self.consume(expect.len());
				ret!(*token);
			}
		}

		// Whitespace
		while matches!(self.peek(0), Some(b' ' | b'\t')) { self.consume(1); }
		if self.pos > start_index { ret!(TokenType::Whitespace); }

		// Name
		{
			// Unbackticked name
			let mut digit_preceding = false;
			let mut can_add_dot = true;

			while let Some(char) = self.peek(0) {
				if char.is_ascii_alphabetic() || char == b'_' { digit_preceding = false; }
				else if char.is_ascii_digit() { digit_preceding = true; } 
				else if digit_preceding && can_add_dot && char == b'.' && self.peek(1).map(|x| x.is_ascii_digit()).unwrap_or(false) {
					(digit_preceding, can_add_dot) = (true, false);
					self.consume(1);
				} else { break; }
				self.consume(1);
			}
			if self.pos > start_index { ret!(TokenType::Name { backticked: false }); }

			// Backticked name
			if start_char == b'`' {
				self.consume(1);
				while let Some(char) = self.peek(0) {
					self.consume(1);
					if char == b'`' { break; }
				}
				ret!(TokenType::Name { backticked: true });
			}
		}

		// String
		{
			let mut num_quotes = 0;
			while matches!(self.peek(num_quotes), Some(b'"')) { num_quotes += 1; }
			
			// Empty short string
			if num_quotes == 2 {
				self.consume(2);
				ret!(TokenType::String { num_quotes });
			}
			
			// Short string
			if num_quotes == 1 {
				self.consume(1);
				let mut escaped = false;
				while let Some(char) = self.peek(0) {
					self.consume(1);
					if escaped { escaped = false; } 
					else if char == b'"' { break; }
				}
				ret!(TokenType::String { num_quotes });
			}
			
			// Raw string
			if num_quotes != 0 {
				self.pos += num_quotes;
				let mut end_quotes = 0;
				while let Some(char) = self.peek(0) {
					self.consume(1);
					if char == b'"' {
						end_quotes += 1;
						if end_quotes == num_quotes { break; }
					} else { end_quotes = 0; }
				}
				ret!(TokenType::String { num_quotes });
			}
		}

		self.pos += 1; // Without this, the tokeniser doesn't move forward.
		ret!(TokenType::Unexpected);
	}
}