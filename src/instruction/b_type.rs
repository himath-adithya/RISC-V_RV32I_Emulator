use crate::{
  cpu::CPU,
  instruction::Instruction,
  instruction_format::InstructionFormat,
};

#[derive(Debug, Clone, Copy)]
pub enum OpcodeB {
  BEQ, BNE, BLT, BGE, BLTU, BGEU,
}

impl Instruction {

  pub fn get_b_instruction(inst_type: &InstructionFormat) -> Instruction {
    let InstructionFormat::B { funct3, rs1, rs2, imm, .. } = *inst_type else {
      panic!("Expected B-type instruction");
    };
    let op: OpcodeB;
    match funct3 {
      0x0 => { op = OpcodeB::BEQ; },
      0x1 => { op = OpcodeB::BNE; },
      0x4 => { op = OpcodeB::BLT; },
      0x5 => { op = OpcodeB::BGE; },
      0x6 => { op = OpcodeB::BLTU; },
      0x7 => { op = OpcodeB::BGEU; },
      _ => panic!("Not a valid instruction!"),
    }
    Instruction::B { op, rs1, rs2, imm }
  }

  pub(crate) fn execute_b(op: OpcodeB, rs1: u8, rs2: u8, imm: u16, cpu: &mut CPU) {
    todo!()
  }

}
