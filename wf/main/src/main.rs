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
	input_file: Option<PathBuf>,

	/// If set, then the output is captured and compared with a nearby `.test`
	/// file of the same name.
	#[arg(long)]
	snapshot_test: bool,

	/// If set, then the output is saved to a `.test` file of the same name.
	#[arg(long)]
	update_snapshot_test: bool
}

fn tokenise(args: TokeniseArgs) {
	let source_text = match args.input_file {
		Some(ref input_file) => fs::read_to_string(input_file).expect("Failed to read source file"),
		None => io::read_to_string(io::stdin()).expect("Failed to read from stdin")
	};
	let mut stdout = String::new();
	for (safety_check, token) in Tokeniser::new(&source_text).enumerate() {
		if safety_check > 99999 { panic!("Safety limit reached"); }
		stdout.push_str(&format!("{},{},{};", token.span.index, token.span.length, token.ty.external_name()));
	}
	// TODO: allow this to be done for whole directories at once.
	if args.snapshot_test {
		let input_file = args.input_file.as_ref().expect("Cannot perform a snapshot test without an input file");
		let snapshot = fs::read_to_string(input_file.with_extension("test")).expect("No snapshot found for this input file");
		if snapshot == stdout {
			println!("✅ Output of tokeniser matches snapshot file");
		} else {
			println!("⚠️ Differences between tokeniser and snapshot file - please review.");
			// TODO: go into specifics
		}
	}
	if args.update_snapshot_test {
		let input_file = args.input_file.as_ref().expect("Cannot perform a snapshot test without an input file");
		fs::write(input_file.with_extension("test"), &stdout).expect("Failed to write to snapshot file");
		println!("✅ Written to snapshot file");
	}
	if !args.snapshot_test && !args.update_snapshot_test {
		print!("{stdout}");
	}
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Commands::Tokenise(args) => tokenise(args)
	}
}