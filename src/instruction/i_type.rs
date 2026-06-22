use crate::{
  cpu::CPU,
  instruction::{Instruction},
  memory::Memory,
  instruction_format::InstructionFormat,
};

#[derive(Debug, Clone, Copy)]
pub enum OpcodeI {
  ADDI, XORI, ORI, ANDI, SLLI, SRLI, SRAI, SLTI, SLTIU, LB, LH, LW, LBU, LHU, JALR, ECALL, EBREAK,
}

impl Instruction {

  pub fn get_i_instruction(inst_type: &InstructionFormat) -> Instruction {
    let InstructionFormat::I {  opcode, rd, funct3, rs1, imm, .. } = *inst_type else {
      panic!("Expected I-type instruction");
    };
    let op: OpcodeI;
    match opcode {
      0x13 => match (funct3, (imm >> 5) & 0x7f) {
        (0x0, _)    => { op = OpcodeI::ADDI;  },
        (0x4, _)    => { op = OpcodeI::XORI;  },
        (0x6, _)    => { op = OpcodeI::ORI;   },
        (0x7, _)    => { op = OpcodeI::ANDI;  },
        (0x1, 0x00) => { op = OpcodeI::SLLI;  },
        (0x5, 0x00) => { op = OpcodeI::SRLI;  },
        (0x5, 0x20) => { op = OpcodeI::SRAI;  },
        (0x2, _)    => { op = OpcodeI::SLTI;  },
        (0x3, _)    => { op = OpcodeI::SLTIU; },
        _           => panic!("Not a valid instruction!"),
      },
      0x03 => match funct3 {
        0x0 => { op = OpcodeI::LB; },
        0x1 => { op = OpcodeI::LH; },
        0x2 => { op = OpcodeI::LW; },
        0x4 => { op = OpcodeI::LBU; },
        0x5 => { op = OpcodeI::LHU; },
        _   => panic!("Not a valid instruction!"),
      },
      0x67 => match funct3 {
        0x0 => { op = OpcodeI::JALR; },
        _   => panic!("Not a valid instruction!"),
      },
      0x73 => match (funct3, imm) {
        (0x0, 0x0) => { op = OpcodeI::ECALL; },
        (0x0, 0x1) => { op = OpcodeI::EBREAK; },
        _          => panic!("Not a valid instruction!"),
      },
      _    => panic!("Not a valid instruction!"),
    }
    Instruction::I { op, rd, rs1, imm }
  }

  pub(crate) fn execute_i(op: OpcodeI, rd: u8, rs1: u8, imm: u16, cpu: &mut CPU, mem: &Memory) {
    let rs1_val = cpu.read_reg(rs1);
    let result = match op {
      OpcodeI::ADDI => rs1_val + imm as u32,
      OpcodeI::XORI => rs1_val ^ imm as u32,
      OpcodeI::ORI => rs1_val | imm as u32,
      OpcodeI::ANDI => rs1_val & imm as u32,
      OpcodeI::SLLI => rs1_val << imm as u32,
      OpcodeI::SRLI => rs1_val >> imm as u32,
      OpcodeI::SRAI => rs1_val >> imm as u32,
      OpcodeI::SLTI => if rs1_val < imm as u32 { 1 } else { 0 },
      OpcodeI::SLTIU => if rs1_val < imm as u32 { 1 } else { 0 },
      // OpcodeI::LB => ,
      // OpcodeI::LH => ,
      // OpcodeI::LW => ,
      // OpcodeI::LBU => , // zero-extend
      // OpcodeI::LHU => , // zero-extend
      // OpcodeI::JLAR => ,
      // OpcodeI::ECALL => ,
      // OpcodeI::EBREAK => ,
      _ => panic!("Not register-immediate instruction!"),
    };
    cpu.write_reg(rd, result);
  }

}
