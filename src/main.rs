// filename of the binary as program argument
// <program_name> <binary_file>
//
// dummy data read from the binary file for testing purposes
// const _DUMMYINSTRUCTIONS: [u32; 6] = [
//     0x93005000, 0x13010000, 0x33011100, 0x9380f0ff, 0xe39c00fe, 0x6f000000,
// ];

mod instruction;
mod cpu;
mod kernel;
mod memory;
mod consts;

use std::env::args;

use crate::{cpu::CPU, kernel::Kernel, memory::Memory};

fn main() {
  // get the binary file name from the command line arguments
  let arg: String = args()
        .nth(1)
        .expect(&format!("Usage: {} <binary_file>", args().nth(0).unwrap_or("<program_name>".into())));

  // initialization and loading program into memory (in a real system kernel also sets the pc)
  let kernel = Kernel::new();
  let mut cpu = CPU::new();
  let mem = Memory {
      bytes: kernel.load_memory(&arg)
  };

  // run the program
  cpu.set_running(true);
  while cpu.is_running() {
      let instruction = cpu.fetch(&mem);
      cpu.inc_pc(); // pc is incremented after fetching
      let instruction = cpu.decode(&instruction); // w rust for allowing variable redeclaration
      instruction.execute(&mut cpu, &mem);
  }
}
