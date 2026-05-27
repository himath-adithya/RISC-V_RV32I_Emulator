use std::convert::TryInto;

use crate::{
  consts::{INST_SIZE, REG_SIZE},
  instruction::Instruction,
  memory::Memory
};

pub struct CPU {
  pc: usize,
  register: [u32; REG_SIZE],
  is_running: bool,
}

impl CPU {
  pub fn fetch(&self, mem: &Memory) -> u32 {
      let pc = self.pc as usize;
      assert!(pc + 4 <= mem.bytes.len(), "PC out bounds: 0x{:08x}", self.pc);
      u32::from_le_bytes(mem.bytes[pc..pc+INST_SIZE].try_into().unwrap())
  }

  pub fn decode(&self, inst: &u32) -> Instruction {
    todo!()
  }

  // register operations
  pub fn read_reg(&self, reg: u32) -> u32 {
    self.register[reg as usize]
  }

  pub fn write_reg(&mut self, reg: u32, value: u32) {
    if reg == 0 {
      // does not raise an exception, it is simply discarded
      return
    }
    self.register[reg as usize] = value;
  }

  // get and set is_running status
  pub fn is_running(&self) -> bool {
    self.is_running
  }

  pub fn set_running(&mut self, is_running: bool) {
    self.is_running = is_running;
  }

  // get, set, and increment pc values
  pub fn pc(&self) -> usize {
    self.pc
  }

  pub fn set_pc(&mut self, pc: usize) {
    self.pc = pc;
  }

  pub fn inc_pc(&mut self) {
    self.pc += INST_SIZE;
  }

  // construction
  pub fn new() -> Self {
      Self {
          pc: 0,
          register: [0; REG_SIZE],
          is_running: false,
      }
  }
}
