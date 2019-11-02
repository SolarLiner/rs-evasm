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
        rule ws() = quiet!{[' ' | '\t']*} / expected!("whitespace")
        rule instr_end() = quiet!{ws() "\n"+ ws()}

        rule dec() -> usize
            = ws() n:$(['0'..='9']+) ws() {n.parse().unwrap()}
        rule hex() -> usize
            = ws() "0x" n:$(['0'..='9' | 'a'..='f' | 'A'..='F']) ws() {
            usize::from_str_radix(n, 16).unwrap()
        }
        rule number() -> usize
            = n:(quiet!{dec() / hex()}) / expected!("number")

        rule instruction() -> &'input str = ws() i:$(quiet!{['a'..='z' | 'A'..='Z']+}) ws() {i} / expected!("instruction")

        rule register() -> u8
            = ws() "R" r:dec() {r as u8} / expected!("register")
        rule constant() -> u16
            = ws() "#" c:number() {c as u16} / expected!("constant")
        rule label() -> &'input str
            = ws() l:$(quiet!{['a'..='z' | 'A'..='Z' | '_'] (['0'..='9' | 'a'..='z' | 'A'..='Z' | '_'])*}) ws() {l}

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
            = l:label() ":" ws() {Instruction::Label(l)} / expected!("label")

        rule instruction_program() -> Instruction<'input>
            = i:(instruction_rr() / instruction_rc() / instruction_rl() / instruction_rp() / instruction_r()) {i} / expected!("instruction")

        pub rule program() -> Vec<Instruction<'input>>
            = l:(instruction_program() / instruction_label()) ** (instr_end()+) (instr_end()*) {l}
    }
}

pub fn parse_input(
    input: &str,
) -> Result<Vec<Instruction>, peg::error::ParseError<peg::str::LineCol>> {
    asm::program(input)
}
