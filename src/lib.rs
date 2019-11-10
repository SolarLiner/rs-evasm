use std::io::Write;

mod generate;
mod parse;

pub fn assemble<W: Write>(input: &str, buf: &mut W) -> Result<usize, String> {
	let ast = parse::parse_input(input).map_err(|e| format!("Parsing error: {}", e))?;
	generate::assemble(ast, buf)
}
