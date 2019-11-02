#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Instruction<'a> {
    Label(&'a str),
    Reg(&'a str, u8),
    RegReg(&'a str, u8, u8),
    RegPtr(&'a str, u8, u8),
    RegCst(&'a str, u8, u16),
    RegLabel(&'a str, u8, &'a str),
}

peg::parser! {
    grammar asm() for str {
        use std::usize;
        use super::Instruction;
        rule ws() = /* quiet!{[' ' | '\t' | '\n']*} */ [' ' | '\t']*
        rule dec() -> usize
            = ws() n:$(['0'..='9']+) ws() {n.parse().unwrap()}
        rule hex() -> usize
            = ws() "0x" n:$(['0'..='9' | 'a'..='f' | 'A'..='F']) ws() {
            usize::from_str_radix(n, 16).unwrap()
        }
        rule number() -> usize
            = n:(dec() / hex()) / expected!("number")
        rule instruction() -> &'input str = ws() i:$(['a'..='z' | 'A'..='Z']+) ws() {i}
        rule constant() -> u16
            = ws() "#" c:number() ws() {c as u16} / expected!("constant")
        rule register() -> u8
            = ws() "R" r:number() ws() {r as u8} / expected!("register")
        rule label() -> &'input str
            = ws() l:$(['a'..='z' | 'A'..='Z' | '_'] (['0'..='9' | 'a'..='z' | 'A'..='Z' | '_'])*) ws() {l}
        rule instruction_r() -> Instruction<'input>
            = i:instruction() r:register() {Instruction::Reg(i, r)}
        rule instruction_rr() -> Instruction<'input>
            = i:instruction() r1:register() "," r2:register() {
            Instruction::RegReg(i, r1, r2)
        }
        rule instruction_rc() -> Instruction<'input>
            = i:instruction() r:register() "," c:constant() {
            Instruction::RegCst(i, r, c)
        }
        rule instruction_rl() -> Instruction<'input>
            = i:instruction() r:register() "," l:label() {
            Instruction::RegLabel(i, r, l)
        }
        rule instruction_rp() -> Instruction<'input>
            = i:instruction() r1:register() "," ws() "[" r2:register() "]" ws() {Instruction::RegPtr(i, r1, r2)}
        rule instruction_label() -> Instruction<'input>
            = l:label() ":" ws() "\n" {Instruction::Label(l)} / expected!("label")

        rule instruction_program() -> Instruction<'input>
            = i:(instruction_r() / instruction_rr() / instruction_rc() / instruction_rl() / instruction_rp()) "\n" {i} / expected!("instruction")

        pub rule program() -> Vec<Instruction<'input>>
            = ws() l:(instruction_program() / instruction_label()) ** ws() ws() {l}
    }
}

pub fn parse_input(
    input: &str,
) -> Result<Vec<Instruction>, peg::error::ParseError<peg::str::LineCol>> {
    asm::program(input)
}
