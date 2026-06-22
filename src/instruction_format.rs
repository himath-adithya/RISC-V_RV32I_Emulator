use crate::instruction::Instruction;

#[derive(Copy, Clone)]
pub enum InstructionFormat {
  R { rd: u8, funct3: u8, rs1: u8, rs2: u8, funct7: u8, },
  I { opcode: u8, rd: u8, funct3: u8, rs1: u8, imm: u16, },
  S { funct3: u8, rs1: u8, rs2: u8, imm: u16, },
  U { opcode: u8, rd: u8, imm: u32, },
  B { funct3: u8, rs1: u8, rs2: u8, imm: u16, },
  J { rd: u8, imm: u32, },
}

impl InstructionFormat {
  pub fn get_inst(inst: &u32, opcode: u8) -> Instruction {
    let inst = *inst;
    match opcode {
      // R-type
      0x33 => {
        let inst_type = InstructionFormat::R {
          rd: ((inst >> 7) & 0x1f) as u8,
          funct3: ((inst >> 12) & 0x07) as u8,
          rs1: ((inst >> 15) & 0x1f) as u8,
          rs2: ((inst >> 20) & 0x1f) as u8,
          funct7: ((inst >> 25) & 0x7f) as u8,
        };
        Instruction::get_r_instruction(&inst_type)
      },
      // I-type
      0x03 | 0x0f | 0x13 | 0x67 | 0x73 => {
        let inst_type = InstructionFormat::I {
          opcode: opcode as u8,
          rd: ((inst >> 7) & 0x01f) as u8,
          funct3: ((inst >> 12) & 0x007) as u8,
          rs1: ((inst >> 15) & 0x01f) as u8,
          imm: ((inst >> 20) & 0xfff) as u16,
        };
        Instruction::get_i_instruction(&inst_type)
      },
      // S-type
      0x23 => {
        let imm: [u32; 2] = [((inst >> 7) & 0x1f), ((inst >> 25) & 0x7f)];
        let inst_type = InstructionFormat::S {
          imm: (imm[0] | (imm[1] << 5)) as u16,
          funct3: ((inst >> 12) & 0x007) as u8,
          rs1: ((inst >> 15) & 0x01f) as u8,
          rs2: ((inst >> 20) & 0x01f) as u8,
        };
        Instruction::get_s_instruction(&inst_type)
      },
      // U-type
      0x17 | 0x37 => {
        let inst_type = InstructionFormat::U {
          opcode: opcode as u8,
          rd: ((inst >> 7) & 0x1f) as u8,
          imm: ((inst >> 12) & 0xfffff) as u32,
        };
        Instruction::get_u_instruction(&inst_type)
      },
      // B-type
      0x63 => {
        let imm: [u32; 4] = [
          (inst >> 8) & 0x0f,  // imm[4:1]
          (inst >> 25) & 0x3f, // imm[10:5]
          (inst >> 7) & 0x01,  // imm[11]
          (inst >> 31 & 0x01), // imm[12]
        ];
        let imm = (imm[0] << 1) | (imm[1] << 5) | (imm[2] << 11) | (imm[3] << 12);
        let imm = (imm as i16 as i32) as u32; // sign-extension
        let inst_type = InstructionFormat::B {
          funct3: ((inst >> 12) & 0x007) as u8,
          rs1: ((inst >> 15) & 0x01f) as u8,
          rs2: ((inst >> 20) & 0x01f) as u8,
          imm: imm as u16,
        };
        Instruction::get_b_instruction(&inst_type)
      }
      // J-type
      0x6f => {
        let imm: [u32; 4] = [
          (inst >> 21) & 0x3ff, // imm[10:1]
          (inst >> 20) & 0x001, // imm[11]
          (inst >> 12) & 0x0ff, // imm[19:12]
          (inst >> 31) & 0x001, // imm[20]
        ];
        let imm = (imm[0] << 1) | (imm[1] << 11) | (imm[2] << 12) | (imm[3] << 20);
        let inst_type = InstructionFormat::J {
          rd: ((inst >> 7) & 0x1f) as u8,
          imm: imm as u32,
        };
        Instruction::get_j_instruction(&inst_type)
      }
      _ => panic!("Unrecognized opcode: 0x{:02x}", opcode),
    }
  }
}
