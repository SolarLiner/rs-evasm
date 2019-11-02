use std::io::{self, Read};

mod parse;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    match parse::parse_input(&input) {
        Ok(ast) => {
            println!("{:?}", ast);
            Ok(())
        }
        Err(err) => {
            eprintln!("{}", err);
            Ok(())
        }
    }
}
