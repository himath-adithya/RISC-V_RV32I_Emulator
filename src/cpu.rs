use std::convert::TryInto;

// CPU is defined below
pub struct Cpu {
    pub pc: u32,
    pub regs: [u32; 32],
    pub mem: Vec<u8>,
    pub running: bool,
}

impl Cpu {
    pub fn fetch(&self) -> u32 {
        let pc = self.pc as usize;
        assert!(pc + 4 <= self.mem.len(), "PC out bounds: 0x{:08x}", self.pc);
        u32::from_le_bytes(self.mem[pc..pc+4].try_into().unwrap())
    }

    pub fn decode(&self) {
        todo!()
    }

    pub fn execute(&self) {
        todo!()
    }
}
