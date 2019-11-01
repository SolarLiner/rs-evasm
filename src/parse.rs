#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Instruction<'a> {
    Label(&'a str),
    RegReg(&'a str, u8, u8),
    RegCst(&'a str, u8, u16),
    RegLabel(&'a str, u8, &'a str)
}

peg::parser! {
    grammar asm() for str {
        use super::Instruction;
        rule number() -> u32
            = n:$(['0'..='9']+) { n.parse().unwrap() }
        rule whitespace() = quiet!{[' ' | '\t' | '\n']*}
        rule instruction<'a>() -> &'a str = i:$(['a'..='z' | 'A'..='Z']) {i}
        rule constant() -> u16
            = "#" c:number() {c.into()}
        rule register() -> u8
            = "R" r:number() {r.into()}
        rule label<'a>() -> Instruction<'a>
            = l:$(['a'..='z' | 'A'..='Z'] (['0'..='9' | 'a'..='z' | 'A'..='Z'])*) {Instruction::Label(l)}
        rule instruction_rr<'a>() -> Instruction<'a>
            = i:instruction() whitespace() r1:register() whitespace() "," whitespace() r2:register() {
            Instruction::RegReg(i, r1, r2)
        }
        rule instruction_rc<'a>() -> Instruction<'a>
            = i:instruction() whitespace() r:register() whitespace() "," whitespace() c:constant() {
            Instruction::RegCst(i, r, c)
        }
        rule instruction_rl<'a>() -> Instruction<'a>
            = i:instruction() whitespace() r:register() whitespace() "," whitespace() l:label() {
            Instruction::RegLabel(i, r, l)
        }

        pub rule program<'a>() -> Vec<Instruction<'a>>
            = l:(label() / instruction_rr() / instruction_rc() / instruction_rl() ** whitespace()) {l}
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Instruction>, peg::error::ParseError<peg::str::LineCol>> {
    asm::program(input)
}