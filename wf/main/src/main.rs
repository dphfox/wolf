use std::{io::{self, BufReader, Read}, time::Instant};

use clap::{Parser, Subcommand};
use wf_parse::explain::explain_parse_error;

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
	let start_time = Instant::now();
	let tokeniser = wf_token::Tokeniser::new(stdin_bytes!());
	let time_to_tokenise = start_time.elapsed();
	let start_time = Instant::now();
	let parser = wf_parse::Parser::new(tokeniser);
	let syntax = parser.collect::<Result<Vec<_>, _>>();
	let time_to_parse = start_time.elapsed();
	match syntax {
		Ok(syntax) => {
			let json = serde_json::to_string_pretty(&syntax).expect("Failed to serialise parser output as JSON");
			print!("{json}");
		},
		Err(err) => explain_parse_error(&err),
	}
	println!("Took {}micros to tokenise and {}micros to parse", time_to_tokenise.as_micros(), time_to_parse.as_micros())
	
}