use crate::{cpu::CPU, memory::Memory};

pub enum Instruction {
  R { opcode: u8, rd: u8, funct3: u8, rs1: u8, rs2: u8, funct7: u8 },
  I { opcode: u8, rd: u8, funct3: u8, rs1: u8, imm: u16 },
  S { opcode: u8, funct3: u8, rs1: u8, rs2: u8, imm: u8 },
  U { opcode: u8, rd: u8, imm: u32 },
  B { opcode: u8, funct3: u8, rs1: u8, rs2: u8, imm: u16 },
  J { opcode: u8, rd: u8, imm: u32 },
}

impl Instruction {
  // implement instruction execute() goes here
  pub fn execute(&self, cpu: &mut CPU, mem: &Memory) {
    match self {
      Instruction::R { opcode, rd, funct3, rs1, rs2, funct7 } => {}
      Instruction::I { opcode, rd, funct3, rs1, imm } => {}
      Instruction::S { opcode, funct3, rs1, rs2, imm } => {}
      Instruction::U { opcode, rd, imm } => {}
      Instruction::B { opcode, funct3, rs1, rs2, imm } => {}
      Instruction::J { opcode, rd, imm } => {}
    }
  }
}
