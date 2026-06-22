use crate::{
  cpu::CPU,
  instruction::{Instruction},
  memory::Memory,
  instruction_format::InstructionFormat,
};

#[derive(Debug, Clone, Copy)]
pub enum OpcodeS {
  SB, SH, SW
}

impl Instruction {

  pub fn get_s_instruction(inst_type: &InstructionFormat) -> Instruction {
    let InstructionFormat::S {  funct3, rs1, rs2, imm, .. } = *inst_type else {
      panic!("Expected S-type instruction");
    };
    let op: OpcodeS;
    match funct3 {
      0x0 => { op = OpcodeS::SB; },
      0x1 => { op = OpcodeS::SH; },
      0x2 => { op = OpcodeS::SW; },
      _   => panic!("Not a valid instruction!"),
    }
    Instruction::S { op, rs1, rs2, imm }
  }

  pub(crate) fn execute_s(op: OpcodeS, rs1: u8, rs2: u8, imm: u16, cpu: &CPU, mem: &mut Memory) {
    todo!()
  }

}
