use std::fs;
use std::io::{self, Read};

mod generate;
mod parse;

fn main() -> io::Result<()> {
	let mut infile = fs::OpenOptions::new().read(true).open("data/prog.evasm")?;
	let mut input = String::new();
	let mut out = fs::OpenOptions::new()
		.write(true)
		.create(true)
		.open("out.evo")?;
	infile.read_to_string(&mut input)?;
	input.make_ascii_uppercase();
	match parse::parse_input(&input) {
		Ok(ast) => match generate::assemble(ast, &mut out) {
			Ok(size) => Ok(println!("Done ({} bytes).", size)),
			Err(e) => {
				eprintln!("Error assembling file: {}", e);
				Err(io::Error::from(io::ErrorKind::Other))
			}
		},
		Err(err) => {
			eprintln!("Parsing error: {}", err);
			Err(io::Error::from(io::ErrorKind::Other))
		}
	}
}
