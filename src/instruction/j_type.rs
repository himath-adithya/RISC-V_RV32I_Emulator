use crate::{
  cpu::CPU,
  instruction::{Instruction},
  instruction_format::InstructionFormat,
};

#[derive(Debug, Clone, Copy)]
pub enum OpcodeJ {
  JAL,
}

impl Instruction {

  pub fn get_j_instruction(inst_type: &InstructionFormat) -> Instruction {
    let InstructionFormat::J { rd, imm, .. } = *inst_type else {
      panic!("Expected J-type instruction");
    };
    Instruction::J { op: OpcodeJ::JAL, rd: rd, imm: imm }
  }

  pub(crate) fn execute_j(op: OpcodeJ, rd: u8, imm: u32, cpu: &mut CPU) {
    match op {
      OpcodeJ::JAL => {},
    }
    todo!()
  }

}
