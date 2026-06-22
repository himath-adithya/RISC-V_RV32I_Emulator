// filename of the binary as program argument
// <program_name> <binary_file>
//
// const _DUMMYINSTRUCTIONS: [u32; 6] = [
//     0x93005000, 0x13010000, 0x33011100, 0x9380f0ff, 0xe39c00fe, 0x6f000000,
// ];

mod consts;
mod cpu;
mod instruction;
mod instruction_format;
mod kernel;
mod memory;

use crate::{cpu::CPU, kernel::Kernel, memory::Memory};
use std::env::args;

fn main() {
  let arg: String = args().nth(1).expect(&format!(
    "Usage: {} <binary_file>",
    args().nth(0).unwrap_or("<program_name>".into())
  ));
  // initialization and loading program into memory (in a real system kernel also sets the pc)
  let kernel = Kernel::new();
  let mut cpu = CPU::new();
  let mut mem = Memory {
    bytes: kernel.load_memory(&arg),
  };
  cpu.set_running(true);
  while cpu.is_running() {
    cpu.fetch(&mem);
    let instruction = cpu.decode();
    cpu.execute(&instruction, &mut mem);
    // cpu.mem()
    // cpu.writeback()
    // cpu.interrupt_handle(&kernel: Kernel)
  }
}
