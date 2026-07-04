use crate::consts::{BYTE_SIZE, HALFWORD_SIZE, INST_SIZE, WORD_SIZE};

// memory stores the instruction as bytes starting from address 0x00
// in an architecture memory inherits the endianess, meanwhile the true representation of values are shown in the registers.
pub struct Bus {
  pub memory: Vec<u8>,
}

impl Bus {

  // SECTION: read_memory

  pub fn read_inst(&self, addr: u32) -> u32 {
    self.bound_check(addr, INST_SIZE);
    u32::from_le_bytes(
      self.memory[addr as usize..addr as usize + INST_SIZE].try_into().unwrap()
    )
  }

  pub fn read_byte(&self, addr: u32) -> u8 {
    self.bound_check(addr, BYTE_SIZE);
    self.memory[addr as usize].try_into().unwrap()
  }

  pub fn read_halfword(&self, addr: u32) -> u16 {
    self.bound_check(addr, HALFWORD_SIZE);
    u16::from_le_bytes(
      self.memory[addr as usize..addr as usize + HALFWORD_SIZE].try_into().unwrap()
    )
  }

  pub fn read_word(&self, addr: u32) -> u32 {
    self.bound_check(addr, WORD_SIZE);
    u32::from_le_bytes(
      self.memory[addr as usize..addr as usize + WORD_SIZE].try_into().unwrap()
    )
  }

  // return an array of u8
  fn bound_check(&self, addr: u32, size: usize) {
    assert!(
      addr as usize + size <= self.memory.len(),
      "Address Misaligned Exception: addr 0x{:08x} out of bounds",
      addr
    );
  }

  // SECTION: write_memory

  pub fn write_byte(&mut self, addr: u32, value: u8) {
    self.write_mem(addr, BYTE_SIZE, value as u32);
  }

  pub fn write_halfword(&mut self, addr: u32, value: u16) {
    self.write_mem(addr, HALFWORD_SIZE, value as u32);
  }

  pub fn write_word(&mut self, addr: u32, value: u32) {
    self.write_mem(addr, WORD_SIZE, value as u32);
  }

  fn write_mem(&mut self, addr: u32, size: usize, value: u32) {
    assert!(
      addr as usize + size <= self.memory.len(),
      "Memory Out of Bounds: addr 0x{:08x} out of bounds",
      addr
    );

    self.memory[(addr as usize)..(addr as usize + size)]
      .copy_from_slice(
        &(value.to_le_bytes())[..size]
      );
  }

}
