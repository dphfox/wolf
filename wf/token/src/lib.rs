// Lossless tokeniser.
// 
// The tokeniser treats all source material as ASCII - UTF-8 characters are not
// specifically part of any syntax, but could still be included in spans like
// names, comments or strings.

use serde::Serialize;
use wf_lookahead::Lookahead;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Span {
	pub index: usize,
	pub length: usize,
	// 1-indexed.
	pub line: usize,
	pub line_index: usize
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Token {
	pub ty: TokenType,
	pub span: Span
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub enum TokenType {
	#[default]
	Unexpected,

	Whitespace,
	Comment,
	Name { name: String },
	String { string: String },

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
	Percent,
	Caret,
	Hash,
	Equal,
	Bang,
	Less,
	More,
	EndLine,
}

macro_rules! exact {
	($s:expr, $t:ident) => { ($s.as_bytes(), Self::$t) };
}

macro_rules! bytes_to_utf8_lossy {
	($content:expr) => {
		String::from_utf8($content).unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).to_string())
	};
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
		exact!("%", Percent),
		exact!("^", Caret),
		exact!("#", Hash),
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
			Comment => "comment",
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
			Percent => "percent",
			Caret => "caret",
			Hash => "hash",
			Equal => "equal",
			Bang => "bang",
			Less => "less",
			More => "more",
			EndLine => "end_line",
		}
	}
}

pub struct Tokeniser<Input: Iterator<Item = u8>> {
    bytes: Lookahead<4, u8, Input>,
	line: usize,
	line_index: usize
}

impl<Input: Iterator<Item = u8>> Tokeniser<Input> {
	pub fn new(input: Input) -> Self {
		Self { bytes: Lookahead::new(input), line: 1, line_index: 1 }
	}
}

impl<Input: Iterator<Item = u8>> Iterator for Tokeniser<Input> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let bytes = &mut self.bytes;
		let start_position = bytes.position();
		let start_line = (self.line, self.line_index);
		macro_rules! ret {
			($ty:expr) => {
				let length = bytes.position() - start_position;
				debug_assert!(length > 0, "Zero length tokens aren't valid - they lead to infinite loops");
				return Some(Token { ty: $ty, span: Span { index: start_position, length, line: start_line.0, line_index: start_line.1 } });
			};
		}
		macro_rules! consume {
			($count:expr) => {{
				let mut line_feed_should_increment = true;
				for char in bytes.consume().take($count) {
					if char == b'\r' {
						self.line += 1;
						self.line_index = 1;
						line_feed_should_increment = false;
						continue;
					} else if char == b'\n' {
						if line_feed_should_increment {
							self.line += 1;
							self.line_index = 1;
						}
						line_feed_should_increment = true;
						continue;
					} else {
						self.line_index += 1;
						line_feed_should_increment = true;
					}
				}
			}};
		}

		// EOF
		let &start_char = bytes.peek(0)?;

		// Comment
		{
			let mut num_hyphens = 0;
			while matches!(bytes.peek(num_hyphens.min(2)), Some(b'-')) {
				num_hyphens += 1;
				// Start consuming behind us if we know we're a long comment to keep within the max lookahead.
				if num_hyphens > 2 { consume!(1); }
			}

			// Long comment
			if num_hyphens > 2 {
				consume!(2); // Consume the two hyphens we didn't initially consume.
				let mut end_hyphens = 0;
				while let Some(&char) = bytes.peek(0) {
					consume!(1);
					if char == b'-' {
						end_hyphens += 1;
						if end_hyphens == num_hyphens { break; }
					} else { end_hyphens = 0; }
				}
				ret!(TokenType::Comment);
			}

			// Short comment
			if num_hyphens == 2 {
				consume!(2);
				while let Some(&char) = bytes.peek(0) && !matches!(char, b'\n' | b'\r') {
					consume!(1);
				}
				ret!(TokenType::Comment);
			}
		}

		// Exact tokens
		for (expect, token) in TokenType::SORTED_EXACT_TOKENS {
			let found_match = expect.iter().enumerate().all(|(offset, &expected)| bytes.peek(offset) == Some(&expected));
			if found_match {
				consume!(expect.len());
				ret!(token.clone());
			}
		}

		// Whitespace
		while matches!(bytes.peek(0), Some(b' ' | b'\t')) { consume!(1); }
		if bytes.position() > start_position { ret!(TokenType::Whitespace); }

		let mut content_u8 = vec![];

		// Name
		{
			// Unbackticked name
			let mut digit_preceding = false;
			let mut can_add_dot = true;

			while let Some(&char) = bytes.peek(0) {
				if char.is_ascii_alphanumeric() || char == b'_' {
					digit_preceding = char.is_ascii_digit();
					consume!(1);
					content_u8.push(char);
					continue;
				} else if digit_preceding && can_add_dot && char == b'.' {
					if let Some(&next_digit) = bytes.peek(1).filter(|x| x.is_ascii_digit()) {
						digit_preceding = true; can_add_dot = false;
						consume!(2);
						content_u8.push(char);
						content_u8.push(next_digit);
						continue;
					}
				}
				break;
			}
			if bytes.position() > start_position { ret!(TokenType::Name { name: bytes_to_utf8_lossy!(content_u8) }); }

			// Backticked name
			if start_char == b'`' {
				consume!(1);
				while let Some(&char) = bytes.peek(0) {
					consume!(1);
					if char == b'`' { break; }
					content_u8.push(char);
				}
				ret!(TokenType::Name { name: bytes_to_utf8_lossy!(content_u8) });
			}
		}

		// String
		{
			let mut num_quotes = 0;
			while let Some(&char) = bytes.peek(num_quotes.min(2)) && matches!(char, b'"') {
				num_quotes += 1;
				// Start consuming behind us if we know we're a raw string to keep within the max lookahead.
				if num_quotes > 2 { consume!(1); }
			}
			
			// Empty short string
			if num_quotes == 2 {
				consume!(2);
				ret!(TokenType::String { string: bytes_to_utf8_lossy!(content_u8) });
			}
			
			// Short string
			if num_quotes == 1 {
				consume!(1);
				let mut escaped = false;
				while let Some(&char) = bytes.peek(0) {
					consume!(1);
					if escaped { escaped = false; } 
					else if char == b'"' { break; }
					content_u8.push(char);
				}
				ret!(TokenType::String { string: bytes_to_utf8_lossy!(content_u8) });
			}
			
			// Raw string
			if num_quotes != 0 {
				consume!(2); // Consume the two hyphens we didn't initially consume.
				let mut end_quotes = 0;
				while let Some(&char) = bytes.peek(0) {
					consume!(1);
					if char == b'"' {
						end_quotes += 1;
						if end_quotes == num_quotes { break; }
					} else { end_quotes = 0; }
					content_u8.push(char);
				}
				content_u8.truncate(content_u8.len() - end_quotes + 1);
				ret!(TokenType::String { string: bytes_to_utf8_lossy!(content_u8) });
			}
		}

		consume!(1); // Without this, the tokeniser doesn't move forward.
		ret!(TokenType::Unexpected);
	}
}