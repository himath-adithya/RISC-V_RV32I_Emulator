use crate::{
  cpu::CPU,
  instruction::{Instruction},
  instruction_format::InstructionFormat,
};

#[derive(Debug, Clone, Copy)]
pub enum OpcodeR {
  ADD, SUB, XOR, OR, AND, SLL, SRL, SRA, SLT, SLTU,
}

impl Instruction {

  pub fn get_r_instruction(inst_type: &InstructionFormat) -> Instruction {
    let InstructionFormat::R { rd, funct3, rs1, rs2, funct7, .. } = *inst_type else {
      panic!("Expected R-type instruction");
    };
    let op: OpcodeR;
    match (funct3, funct7) {
      (0x0, 0x00) => { op = OpcodeR::ADD;  },
      (0x0, 0x20) => { op = OpcodeR::SUB;  },
      (0x4, 0x00) => { op = OpcodeR::XOR;  },
      (0x6, 0x00) => { op = OpcodeR::OR;   },
      (0x7, 0x00) => { op = OpcodeR::AND;  },
      (0x1, 0x00) => { op = OpcodeR::SLL;  },
      (0x5, 0x00) => { op = OpcodeR::SRL;  },
      (0x5, 0x20) => { op = OpcodeR::SRA;  },
      (0x2, 0x00) => { op = OpcodeR::SLT;  },
      (0x3, 0x10) => { op = OpcodeR::SLTU; },
      _ => panic!("Not a valid instruction!"),
    };
    Instruction::R { op, rd, rs1, rs2 }
  }

  pub(crate) fn execute_r(op: OpcodeR, rd: u8, rs1: u8, rs2: u8, cpu: &mut CPU) {
    let rs1_val = cpu.read_reg(rs1);
    let rs2_val = cpu.read_reg(rs2);
    let result = match op {
      OpcodeR::ADD => rs1_val.wrapping_add(rs2_val),
      OpcodeR::SUB => rs1_val.wrapping_sub(rs2_val),
      OpcodeR::XOR => rs1_val ^ rs2_val,
      OpcodeR::OR => rs1_val | rs2_val,
      OpcodeR::AND => rs1_val & rs2_val,
      OpcodeR::SLL => rs1_val << rs2_val,
      OpcodeR::SRL => rs1_val >> rs2_val,
      OpcodeR::SRA => rs1_val >> rs2_val,
      OpcodeR::SLT => if rs1_val < rs2_val { 1 } else { 0 },
      OpcodeR::SLTU => if rs1_val < rs2_val { 1 } else { 0 },
    };
    cpu.write_reg(rd, result);
  }

}
