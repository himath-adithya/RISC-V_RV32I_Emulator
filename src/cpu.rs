use crate::{
  consts::{INST_SIZE, REG_SIZE},
  instruction::Instruction,
  instruction_format::InstructionFormat,
  memory::Memory,
};

pub struct CPU {
  pc: usize, // holds the relative address of the next instruction
  ir: u32, // holds the current fetched instruction
  gpr: [u32; REG_SIZE], // holds general register values
  is_running: bool,
}

impl CPU {

  pub fn fetch(&mut self, mem: &Memory) {
    let pc   = self.pc();
    let inst = mem.read_inst(pc as u32);
    self.set_ir(inst); // store the instruction in IR (Instruction Register)
    self.inc_pc(); // pc is incremented after fetching
  }

  // NOTE: in a real world cpu decode stage fetches the opcode and fetches the operands, also in some cases sign extends
  pub fn decode(&self) -> Instruction {
    let inst = self.get_ir();
    let opcode = inst & 0x7f;
    InstructionFormat::get_inst(&inst, opcode as u8)
  }

  // NOTE: implement sign extension in immediates
  pub fn execute(&mut self, inst: &Instruction, mem: &mut Memory) {
    (*inst).execute(mem, self);
  }

  // register operations
  pub fn read_reg(&self, reg: u8) -> u32 {
    self.gpr[reg as usize]
  }

  pub fn write_reg(&mut self, reg: u8, value: u32) {
    match reg {
      0 => return, // does not raise an exception, it is simply discarded
      _ => self.gpr[reg as usize] = value,
    }
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

  // fn set_pc(&mut self, pc: usize) {
  //   self.pc = pc;
  // }

  fn inc_pc(&mut self) {
    self.pc += INST_SIZE;
  }

  fn get_ir(&self) -> u32 {
    self.ir
  }

  fn set_ir(&mut self, inst: u32) {
    self.ir = inst;
  }

  // construction
  pub fn new() -> Self {
    Self {
      pc: 0,
      gpr: [0; REG_SIZE],
      is_running: false,
      ir: 0,
    }
  }

}
