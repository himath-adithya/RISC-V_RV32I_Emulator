// memory stores the instruction as bytes starting from address 0x00
pub struct Memory {
  pub bytes: Vec<u8>,
}

impl Memory {
  pub fn read_word(&self, addr: u32) -> u32 {
    todo!();
  }

  pub fn read_halfword(&self, addr: u32) -> u32 {
    todo!();
  }

  pub fn read_byte(&self, addr: u32) -> u32 {
    todo!();
  }
}
