fn main() {
    create_hello_word();
}

pub fn create_hello_word() {
    let program: Vec<u8> = vec![
        0x3000, // Start address
        0xE002, // LEA 0 1. Set R0 to the address where the "Hello world!" string starts.
        0xF024, // TRAP PUTSP. Print the string starting at the address in R0 to stdout.
        0xF025, // TRAP HALT. Stop the program
        0x6548, 0x6C6C, 0x206F, 0x6F77, 0x6C72, 0x2164,
        0x000A, // "Hello world!\n" string with its null terminator.
    ]
    .into_iter()
    .map(|a| (a as u16).to_be_bytes())
    .flatten()
    .collect();

    std::fs::write("./examples/hello_world.obj", program).unwrap();
}
