// filename of the binary as program argument
// <program_name> <binary_file>
use std::env as env;
use std::fs as fs;

// dummy data read from the binary file
// TODO: should be converted to vector of bytes
const _DUMMYINSTRUCTIONS: [u32; 6] = [0x93005000, 0x13010000, 0x33011100, 0x9380f0ff, 0xe39c00fe, 0x6f000000];

fn main() {
  // load the binary file into a byte vector
  let prg = env::args().nth(0).expect("Program name not found"); // Get the program name for usage message
  let arg = env::args().nth(1).expect(&format!("Usage: {prg} <binary_file>")); // Get the binary file name from the command line arguments
  let file = fs::read(arg).expect("Failed to read the binary file"); // Read the binary file into a byte vector
  println!("Read {} bytes from the binary file", file.len());

  // convert the byte vector to a vector of u32 instructions
  let mut _instructions: Vec<u32> = file.chunks(4).map(|chunk| {
    let mut arr = [0u8; 4];
    arr.copy_from_slice(chunk);
    u32::from_le_bytes(arr)
  }).collect();
}