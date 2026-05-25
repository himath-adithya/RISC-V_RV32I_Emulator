// load memory into the CPU's memory
pub fn load_memory(arg: &String) -> Vec<u8> {
    let mem = std::fs::read(arg).expect("Failed to read the binary file");
    // NOTE: this line below is not of real logic, therefore can be removed
    if mem.len() % 4 != 0 {
        panic!("Memory size is not a multiple of 4");
    }
    mem
}
