// handles syscalls and loading program into memory
use crate::{consts::INST_SIZE, cpu::CPU};

pub struct Kernel;

impl Kernel {
  pub fn load_memory(&self, arg: &String) -> Vec<u8> {
      let mem = std::fs::read(arg).expect("Failed to read the binary file");
      // NOTE: this line below is not of real logic, therefore can be removed
      if mem.len() % INST_SIZE != 0 {
          panic!("Memory size is not a multiple of 4");
      }
      mem
  }

  fn handle_ecall(&mut self, cpu: &mut CPU) {
    match cpu.read_reg(17) { // a7 register
      0 => {
        cpu.write_reg(10, 0);
      }
      1 => {
        cpu.write_reg(10, 1);
      }
      2 => {
        cpu.write_reg(10, 2);
      }
      10 => {
        cpu.set_running(false);
      }
      _ => {}
    }
  }

  fn handle_exception(cpu: &mut CPU, cause: u32) {
    match cause {
      _ => panic!("Unknown exception at PC: 0x{:08x}", cpu.pc())
    }
  }

  // construction
  pub fn new() -> Self {
    Self {}
  }
}
