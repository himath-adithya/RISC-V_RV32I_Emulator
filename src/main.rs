// filename of the binary as program argument
// <program_name> <binary_file>
//
// dummy data read from the binary file for testing purposes
// const _DUMMYINSTRUCTIONS: [u32; 6] = [
//     0x93005000, 0x13010000, 0x33011100, 0x9380f0ff, 0xe39c00fe, 0x6f000000,
// ];

fn main() {
    // load the binary file into a byte vector
    let prg: String = std::env::args().nth(0).expect("Program name not found"); // Get the program name for usage message
    let arg: String = std::env::args()
        .nth(1)
        .expect(&format!("Usage: {prg} <binary_file>")); // Get the binary file name from the command line arguments
    let mem: Vec<u8> = std::fs::read(arg).expect("Failed to read the binary file");
    // error if memory.len() % 4 != 0
    if mem.len() % 4 != 0 {
        panic!("Memory size is not a multiple of 4");
    }
    // convert the byte vector to a vector of u32 instructions
    let insts: Vec<u32> = mem
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes(std::convert::TryInto::try_into(chunk).unwrap()))
        .collect();

    // NOTE: this will be removed in the future
    print!("Instructions read from the binary file:\n");
    for (i, inst) in insts.iter().enumerate() {
        println!("Instruction {}: 0x{:08x}", i, inst);
    }

    // a method should be implemented to execute the instructions in the vector
    // for now, we will just print the instructions
}
