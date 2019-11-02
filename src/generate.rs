use std::io::Write;

#[allow(exceeding_bitshifts)]
use crate::parse::Instruction;

pub fn assemble<W: Write>(instructions: Vec<Instruction>, buf: &mut W) -> Result<usize, String> {
	let mut written = 0usize;
	for inst in Instruction::set_addresses(instructions) {
		let opcode = binarize(inst).ok_or(format!("Cannot convert to binary opcode: {}", inst))?;
		written += buf
			.write(&opcode)
			.map_err(|e| format!("Cannot write into buffer: {}", e))?;
	}
	Ok(written)
}

fn binarize(inst: Instruction) -> Option<[u8; 4]> {
	match get_code(&inst) {
		Some((code, reset, flag, offset)) => match inst {
			Instruction::RegReg(_, r1, r2) | Instruction::RegPtr(_, r1, r2) => {
				Some(make_opcode_reg_reg(code, reset, flag, offset, r1, r2))
			}
			Instruction::RegCst(_, reg, cst) => {
				Some(make_opcode_reg_cst(code, reset, flag, offset, reg, cst))
			}
			Instruction::Reg(_, reg) => {
				Some(make_opcode_reg_reg(code, reset, flag, offset, reg, 0))
			}
			_ => None,
		},
		None => None,
	}
}

fn get_code(inst: &Instruction) -> Option<(u8, bool, bool, u8)> {
	match inst {
		Instruction::RegReg("ADD", _, _) => Some((0b0000, false, false, 0)),
		Instruction::RegCst("ADD", _, _) => Some((0b0001, false, false, 0)),
		Instruction::RegReg("ADDC", _, _) => Some((0b0000, false, true, 0)),
		Instruction::RegCst("ADDC", _, _) => Some((0b0001, false, true, 0)),
		Instruction::RegReg("SUB", _, _) => Some((0b0011, false, false, 0)),
		Instruction::RegCst("SUB", _, _) => Some((0b0111, false, false, 0)),
		Instruction::RegReg("SUBC", _, _) => Some((0b0011, false, true, 0)),
		Instruction::RegCst("SUBC", _, _) => Some((0b0111, false, true, 0)),
		Instruction::RegReg("MOV", _, _) => Some((0b0000, true, false, 0)),
		Instruction::RegCst("MOV", _, _) => Some((0b0001, true, false, 0)),
		Instruction::Reg("PUSH", _) => Some((0b0010, false, false, 0)),
		Instruction::Reg("POP", _) => Some((0b0010, true, false, 0)),
		Instruction::RegPtr("LDR", _, _) => Some((0b0100, false, false, 0)),
		Instruction::RegCst("LDR", _, _) => Some((0b0101, true, false, 0)),
		Instruction::RegPtr("STR", _, _) => Some((0b1000, true, false, 0)),
		Instruction::RegCst("STR", _, _) => Some((0b1001, true, false, 0)),
		Instruction::RegReg("CMP", _, _) => Some((0b1100, true, true, 0)),
		Instruction::RegCst("CMP", _, _) => Some((0b1100, true, true, 1)),
		Instruction::Reg("BEQ", _) => Some((0b1011, true, false, 0)),
		Instruction::Reg("BNEQ", _) => Some((0b1011, true, true, 0)),
		Instruction::Reg("BLT", _) => Some((0b1011, true, true, 1)),
		Instruction::Reg("BLE", _) => Some((0b1011, true, false, 1)),
		Instruction::Reg("IN", _) => Some((0b1111, true, false, 1)),
		Instruction::Reg("OUT", _) => Some((0b1111, true, true, 1)),
		_ => None,
	}
}

#[inline]
fn make_opcode_reg_reg(code: u8, reset: bool, flag: bool, offset: u8, op1: u8, op2: u8) -> [u8; 4] {
	[
		((code & 0xF) << 4) as u8
			| ((reset as u8) << 3) as u8
			| ((flag as u8) << 2) as u8
			| (offset & 0b11) as u8,
		(op1 & 0xF),
		(op2 & 0xF) << 4,
		0u8,
	]
}

#[inline]
fn make_opcode_reg_cst(
	code: u8,
	reset: bool,
	flag: bool,
	offset: u8,
	op1: u8,
	op2: u16,
) -> [u8; 4] {
	[
		((code & 0xF) << 4) as u8
			| ((reset as u8) << 3) as u8
			| ((flag as u8) << 2) as u8
			| (offset & 0b11) as u8,
		(op1 & 0xF),
		((op2 & 0xFF00) >> 8) as u8,
		(op2 & 0xFF) as u8,
	]
}
