use std::{fs, io, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use wf_token::Tokeniser;

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
	Tokenise(TokeniseArgs)
}

#[derive(Args, Clone)]
/// Directly tokenises a source file and emits tokens to stdout.
struct TokeniseArgs {
	/// The file that should be tokenised. Recommended to be a `.wf` file.
	/// If none is provided, the input is read from stdin.
	input_file: Option<PathBuf>
}

fn tokenise(args: TokeniseArgs) {
	let source_text = match args.input_file {
		Some(input_file) => fs::read_to_string(input_file).expect("Failed to read source file"),
		None => io::read_to_string(io::stdin()).expect("Failed to read from stdin")
	};
	for token in Tokeniser::new(&source_text) {
		print!("{},{},{};", token.span.index, token.span.length, token.ty.external_name());
	}
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Commands::Tokenise(args) => tokenise(args)
	}
}