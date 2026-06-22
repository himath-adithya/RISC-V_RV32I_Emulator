use crate::consts::{BYTE_SIZE, HALFWORD_SIZE, INST_SIZE, WORD_SIZE};

// memory stores the instruction as bytes starting from address 0x00
pub struct Memory {
  pub bytes: Vec<u8>,
}

impl Memory {

  pub fn read_inst(&self, addr: u32) -> u32 {
    self.read_mem(addr, INST_SIZE) as u32
  }

  // pub fn read_word(&self, addr: u32) -> u32 {
  //   self.read_mem(addr, WORD_SIZE) as u32
  // }

  // pub fn read_halfword(&self, addr: u32) -> u16 {
  //   self.read_mem(addr, HALFWORD_SIZE) as u16
  // }

  // pub fn read_byte(&self, addr: u32) -> u8 {
  //   self.read_mem(addr, BYTE_SIZE) as u8
  // }

  // pub fn write_word(&mut self, addr: u32, value: u32) {
  //   self.write_mem(addr, WORD_SIZE, value as usize);
  // }

  // pub fn write_halfword(&mut self, addr: u32, value: u32) {
  //   self.write_mem(addr, HALFWORD_SIZE, value as usize);
  // }

  // pub fn write_byte(&mut self, addr: u32, value: u32) {
  //   self.write_mem(addr, BYTE_SIZE, value as usize);
  // }

  fn read_mem(&self, addr: u32, size: usize) -> usize {
    assert!(
      addr as usize + size <= self.bytes.len(),
      "addr 0x{:08x} out of bounds",
      addr
    );
    usize::from_le_bytes(
      self.bytes[(addr as usize)..(addr as usize + size)]
        .try_into()
        .unwrap(),
    )
  }

  // fn write_mem(&mut self, addr: u32, size: usize, value: usize) {
  //   assert!(
  //     addr as usize + size <= self.bytes.len(),
  //     "addr 0x{:08x} out of bounds",
  //     addr
  //   );
  //   self.bytes[(addr as usize)..(addr as usize + size)].copy_from_slice(&value.to_le_bytes());
  // }

}
