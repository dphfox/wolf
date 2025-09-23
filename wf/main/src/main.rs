use std::io;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
/// Process source files written in the Wolf programming language, using the
/// Wolf Reference Implementation.
struct Cli {
	#[command(subcommand)]
	command: Commands
}

#[derive(Subcommand, Clone)]
enum Commands {
	/// Tokenises a source file byte-for-byte from stdin, and emits tokens to stdout.
	/// 
	/// Tokenisation happens as ASCII and metadata like Byte Order Marks are not
	/// removed automatically.
	Tokenise,
	
	/// Tokenises and parses a sourcefile from stdin and emits a JSON syntax tree to stdout.
	/// 
	/// See `tokenise` for information about tokenisation.
	Parse
}

fn tokenise() {
	let source_text = io::read_to_string(io::stdin()).expect("Failed to read from stdin");
	let tokeniser = wf_token::Tokeniser::new(&source_text);
	let mut stdout = String::new();
	for (safety_check, token) in tokeniser.enumerate() {
		if safety_check > 99999 { panic!("Safety limit reached"); }
		stdout.push_str(&format!("{},{},{};", token.span.index, token.span.length, token.ty.external_name()));
	}
	print!("{stdout}");
}

fn parse() {
	let source_text = io::read_to_string(io::stdin()).expect("Failed to read from stdin");
	let tokeniser = wf_token::Tokeniser::new(&source_text);
	let parser = wf_parse::Parser::new(tokeniser);
	let syntax = parser.collect::<Result<Vec<_>, _>>();
	let json = serde_json::to_string(&syntax).expect("Failed to serialise parser output as JSON");
	print!("{json}");
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Commands::Tokenise => tokenise(),
		Commands::Parse => parse()
	}
}