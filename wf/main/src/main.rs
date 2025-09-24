use std::io::{self, Read, BufReader};

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

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Commands::Tokenise => tokenise(),
		Commands::Parse => parse()
	}
}

macro_rules! stdin_bytes {
	() => {
		BufReader::new(io::stdin()).bytes().map(|x| x.expect("Failed to read stdin"))
	};
}

fn tokenise() {
	let tokeniser = wf_token::Tokeniser::new(stdin_bytes!());
	for token in tokeniser {
		print!("{},{},{};", token.span.index, token.span.length, token.ty.external_name());
	}
}

fn parse() {
	let tokeniser = wf_token::Tokeniser::new(stdin_bytes!());
	let parser = wf_parse::Parser::new(tokeniser);
	// let syntax = parser.collect::<Result<Vec<_>, _>>();
	let syntax = parser.filter_map(Result::ok).collect::<Vec<_>>();
	let json = serde_json::to_string(&syntax).expect("Failed to serialise parser output as JSON");
	print!("{json}");
}