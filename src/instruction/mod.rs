mod r_type;
mod i_type;
mod s_type;
mod b_type;
mod j_type;
mod u_type;

use crate::{
  cpu::CPU,
  instruction::{
    b_type::OpcodeB,
    i_type::OpcodeI,
    j_type::OpcodeJ,
    r_type::OpcodeR,
    s_type::OpcodeS,
    u_type::OpcodeU
  },
  kernel::Kernel,
  bus::Bus
};

pub enum Instruction {
  R { op: OpcodeR, rd: u8,  rs1: u8, rs2: u8  },
  I { op: OpcodeI, rd: u8,  rs1: u8, imm: u16 },
  S { op: OpcodeS, rs1: u8, rs2: u8, imm: u16 },
  B { op: OpcodeB, rs1: u8, rs2: u8, imm: u16 },
  J { op: OpcodeJ, rd: u8,  imm: u32          },
  U { op: OpcodeU, rd: u8,  imm: u32          },
}

impl Instruction {

  pub fn execute(&self, mem: &mut Bus, cpu: &mut CPU, kernel: &mut Kernel) {
    match *self {
      Instruction::R { op, rd, rs1, rs2 } => {
        Self::execute_r(op, rd, rs1, rs2, cpu);
      }
      Instruction::I { op, rd, rs1, imm } => {
        Self::execute_i(op, rd, rs1, imm, cpu, mem, kernel);
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
