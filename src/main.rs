// filename of the binary as program argument
// <program_name> <binary_file>
//
// dummy data read from the binary file for testing purposes
// const _DUMMYINSTRUCTIONS: [u32; 6] = [
//     0x93005000, 0x13010000, 0x33011100, 0x9380f0ff, 0xe39c00fe, 0x6f000000,
// ];

mod cpu;
mod functions;

fn main() {
    // load the binary file into a byte vector
    let prg: String = std::env::args().nth(0).expect("Program name not found"); // Get the program name for usage message
    let arg: String = std::env::args()
        .nth(1)
        .expect(&format!("Usage: {prg} <binary_file>")); // Get the binary file name from the command line arguments

    let mut cpu = crate::cpu::Cpu {
        pc: 0,
        regs: [0; 32],
        mem: Vec::new(),
        running: false,
    };

    cpu.mem = crate::functions::load_memory(&arg);
    cpu.mem = std::fs::read(arg).expect("Failed to read the binary file");

    // a method should be implemented to execute the instructions in the vector
    // for now, we will just print the instructions
}
