// Lossless tokeniser.
// 
// The tokeniser treats all source material as ASCII - UTF-8 characters are not
// specifically part of any syntax, but could still be included in spans like
// names, comments or strings.
// 
// The implementation is intentionally simple for maintainability and for high
// performance. The cost of this is that a whole file is tokenised at once.

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

pub struct Tokeniser<'a> {
	ascii_chars: &'a [u8],
	pos: usize,
}

impl<'a> Tokeniser<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			ascii_chars: input.as_bytes(),
			pos: 0
		}
	}
}

impl Iterator for Tokeniser<'_> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let start_index = self.pos;
		let len = self.ascii_chars.len();
		let chars = self.ascii_chars;
		macro_rules! ret {
			($ty:expr) => {
				let length = self.pos - start_index;
				debug_assert!(length > 0, "Zero length tokens aren't valid - they lead to infinite loops");
				return Some(Token { ty: $ty, span: Span { index: start_index, length } });
			};
		}

		// EOF
		if start_index >= len { return None; }

		// Comment
		{
			let mut num_hyphens = 0;
			while self.pos + num_hyphens < len && chars[self.pos + num_hyphens] == b'-' { num_hyphens += 1; }

			// Long comment
			if num_hyphens > 2 {
				self.pos += num_hyphens;
				let mut end_hyphens = 0;
				while self.pos < len {
					if chars[self.pos] == b'-' {
						end_hyphens += 1;
						if end_hyphens == num_hyphens { self.pos += 1; break; }
					} else { end_hyphens = 0; }
					self.pos += 1;
				}
				ret!(TokenType::Comment { num_hyphens });
			}

			// Short comment
			if num_hyphens == 2 {
				self.pos += 2;
				while self.pos < len && !matches!(chars[self.pos], b'\n' | b'\r') { self.pos += 1; }
				ret!(TokenType::Comment { num_hyphens });
			}
		}

		// Exact tokens
		for (expect, token) in TokenType::SORTED_EXACT_TOKENS {
			let (from, to) = (self.pos, self.pos + expect.len());
			// Because `to` is exclusive, it must be <= len, not < len.
			if to <= len && chars[from..to] == **expect {
				self.pos += expect.len();
				ret!(*token);
			}
		}

		// Whitespace
		while self.pos < len && matches!(chars[self.pos], b' ' | b'\t') { self.pos += 1; }
		if self.pos > start_index { ret!(TokenType::Whitespace); }

		// Name
		{
			// Unbackticked name
			let mut digit_preceding = false;
			let mut can_add_dot = true;
			while self.pos < len {
				let char = chars[self.pos];
				if char.is_ascii_alphabetic() || char == b'_' { 
					digit_preceding = false; 
					self.pos += 1; 
				} else if char.is_ascii_digit() {
					digit_preceding = true;
					self.pos += 1; 
				} else if digit_preceding && can_add_dot && char == b'.' && self.pos + 1 < len && chars[self.pos + 1].is_ascii_digit() {
					(digit_preceding, can_add_dot) = (true, false);
					self.pos += 2;
				} else { break; }
			}
			if self.pos > start_index { ret!(TokenType::Name { backticked: false }); }

			// Backticked name
			if self.pos < len && chars[self.pos] == b'`' {
				self.pos += 1;
				while self.pos < len && chars[self.pos] != b'`' { self.pos += 1; }
				if self.pos < len && chars[self.pos] == b'`' { self.pos += 1; }
				ret!(TokenType::Name { backticked: true });
			}
		}

		// String
		{
			let mut num_quotes = 0;
			while self.pos + num_quotes < len && chars[self.pos + num_quotes] == b'"' { num_quotes += 1; }
			
			// Empty short string
			if num_quotes == 2 {
				self.pos += 2;
				ret!(TokenType::String { num_quotes });
			}
			
			// Short string
			if num_quotes == 1 {
				self.pos += 1;
				let mut escaped = false;
				while self.pos < len {
					if escaped {
						escaped = false;
						self.pos += 1;
					} else if chars[self.pos] == b'"' {
						self.pos += 1;
						break;
					} else { self.pos += 1; }
				}
				ret!(TokenType::String { num_quotes });
			}
			
			// Raw string
			if num_quotes != 0 {
				self.pos += num_quotes;
				let mut end_hyphens = 0;
				while self.pos < len {
					if chars[self.pos] == b'"' {
						end_hyphens += 1;
						if end_hyphens == num_quotes { self.pos += 1; break; }
					} else { end_hyphens = 0; }
					self.pos += 1;
				}
				ret!(TokenType::String { num_quotes });
			}
		}

		self.pos += 1; // Without this, the tokeniser doesn't move forward.
		ret!(TokenType::Unexpected);
	}
}