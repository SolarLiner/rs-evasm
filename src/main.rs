use std::io::{self, Read};

use clap::{App, Arg};

// Re-exports
pub use generate::assemble;

use crate::utils::{Input, Output};

pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_ENV");

mod generate;
mod parse;
mod utils;

fn main() -> io::Result<()> {
	let matches = App::new("evasm")
		.help("Assemble Eva ASM into an executable")
		.author("Nathan G. <solarliner@gmail.com>")
		.arg(
			Arg::with_name("input")
				.index(1)
				.help("Input file (default: stdin)"),
		)
		.arg(
			Arg::with_name("output")
				.short("o")
				.long("output")
				.takes_value(true)
				.help("Output file (default: stdout)"),
		)
		.get_matches();
	let mut input = Input::from_arg(matches.value_of("input"))?;
	let mut output = Output::from_arg(matches.value_of("output"))?;
	let mut input_str = String::new();
	input.read_to_string(&mut input_str)?;
	input_str.make_ascii_uppercase();
	match parse::parse_input(&input_str) {
		Ok(ast) => match generate::assemble(ast, &mut output) {
			Ok(size) => Ok(eprintln!("✅ Done ({} bytes).", size)),
			Err(e) => {
				eprintln!("Error while assembling into executable: {}", e);
				Err(io::Error::from(io::ErrorKind::Other))
			}
		},
		Err(err) => {
			eprintln!("Parsing error: {}", err);
			Err(io::Error::from(io::ErrorKind::Other))
		}
	}
}
