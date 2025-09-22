use std::{fs::read_to_string, path::PathBuf};

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
	input_file: PathBuf
}

fn tokenise(args: TokeniseArgs) {
	let source_text = read_to_string(args.input_file).expect("Failed to read source file");
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