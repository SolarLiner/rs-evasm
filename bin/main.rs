use std::io::Read;

use clap::{App, Arg};
use wbuf::{Input, Output};

fn main() -> std::io::Result<()> {
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

    let mut input_file = Input::from_arg(matches.value_of("input"))?;
    let mut output_file = Output::from_arg(matches.value_of("output"))?;
    let mut input_str = String::new();
    input_file.read_to_string(&mut input_str)?;
    match evasm::assemble(&input_str, &mut output_file) {
        Ok(size) => Ok(eprintln!("✔ Done ({} bytes).", size)),
        Err(err) => {
            eprintln!("❌ Error: {}", err);
            Err(std::io::Error::from(std::io::ErrorKind::Other))
        }
    }
}