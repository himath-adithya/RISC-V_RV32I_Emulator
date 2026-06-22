// handles syscalls and loading program into memory
// use crate::{cpu::CPU};

pub struct Kernel;

impl Kernel {

  pub fn load_memory(&self, arg: &String) -> Vec<u8> {
    let mem = std::fs::read(arg).expect("Failed to read the binary file");
    mem
  }

  // fn handle_ecall(&mut self, cpu: &mut CPU) {
  //   match cpu.read_reg(17) {
  //     // a7 register holds syscall number
  //     63 => {
  //       // print the integer in a0
  //       print!("{}", cpu.read_reg(10));
  //     }
  //     93 => {
  //       // exit the program
  //       cpu.set_running(false);
  //       print!("the program has finished its execution phase");
  //     }
  //     _ => panic!("Unkown syscall was invoked at PC: 0x{:08x}", cpu.pc()),
  //   }
  // }

  // fn handle_exception(cpu: &mut CPU, cause: u32) {
  //   match cause {
  //     _ => panic!("Unknown exception at PC: 0x{:08x}", cpu.pc()),
  //   }
  // }

  pub fn new() -> Self {
    Self {}
  }

}
