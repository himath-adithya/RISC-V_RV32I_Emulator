use crate::{
  cpu::CPU,
  instruction::Instruction,
  instruction_format::InstructionFormat,
};

#[derive(Debug, Clone, Copy)]
pub enum OpcodeU {
  LUI, AUIPC,
}

impl Instruction {

  pub fn get_u_instruction(inst_type: &InstructionFormat) -> Instruction {
    let InstructionFormat::U { opcode, rd, imm } = *inst_type else {
      panic!("Expected U-type instruction");
    };
    let op: OpcodeU;
    match opcode {
      0x67 => { op = OpcodeU::LUI; },
      0x17 => { op = OpcodeU::AUIPC; },
      _ => panic!("Not a valid instruction!"),
    }
    Instruction::U { op, rd, imm }
  }

  pub(crate) fn execute_u(op: OpcodeU, rd: u8, imm: u32, cpu: &mut CPU) {
    match op {
      OpcodeU::LUI => {},
      OpcodeU::AUIPC => {},
    }
    todo!()
  }
}
