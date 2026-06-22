mod r_type;
mod i_type;
mod s_type;
mod b_type;
mod j_type;
mod u_type;

use crate::{cpu::CPU, instruction::{i_type::OpcodeI, r_type::OpcodeR, s_type::OpcodeS, b_type::OpcodeB, j_type::OpcodeJ, u_type::OpcodeU}, memory::Memory};

pub enum Instruction {
  R { op: OpcodeR, rd: u8, rs1: u8, rs2: u8 },
  I { op: OpcodeI, rd: u8, rs1: u8, imm: u16 },
  S { op: OpcodeS, rs1: u8, rs2: u8, imm: u16 },
  B { op: OpcodeB, rs1: u8, rs2: u8, imm: u16 },
  J { op: OpcodeJ, rd: u8, imm: u32 },
  U { op: OpcodeU, rd: u8, imm: u32 },
}

impl Instruction {

  pub fn execute(&self, mem: &mut Memory, cpu: &mut CPU) {
    match *self {
      Instruction::R { op, rd, rs1, rs2 } => {
        Self::execute_r(op, rd, rs1, rs2, cpu);
      }
      Instruction::I { op, rd, rs1, imm } => {
        Self::execute_i(op, rd, rs1, imm, cpu, mem);
      }
      Instruction::S { op, rs1, rs2, imm } => {
        Self::execute_s(op, rs1, rs2, imm, cpu, mem);
      }
      Instruction::B { op, rs1, rs2, imm } => {
        Self::execute_b(op, rs1, rs2, imm, cpu);
      }
      Instruction::J { op, rd, imm } => {
        Self::execute_j(op, rd, imm, cpu);
      }
      Instruction::U { op, rd, imm } => {
        Self::execute_u(op, rd, imm, cpu);
      }
    }
  }

}
