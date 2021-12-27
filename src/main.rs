use nix::sys::signal;
use std::env;
use termios::*;

mod state;
use state::State;

mod opcode;
use opcode::Opcode;

const PC_START: u16 = 0x3000;

fn main() {
    let mut state = State::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: lc3 [image-file1] [image-file2] ...");
        std::process::exit(0);
    } else {
        for file_path in &args[1..] {
            load_image_file(&mut state, file_path).unwrap();
        }
    }

    let sig_action = signal::SigAction::new(
        signal::SigHandler::Handler(handle_interrupt),
        signal::SaFlags::empty(),
        signal::SigSet::empty(),
    );

    disable_input_buffering();
    unsafe {
        signal::sigaction(signal::Signal::SIGINT, &sig_action).unwrap();
    }

    state.pc = PC_START;
    while state.running {
        let instruction = state.mem_read(state.pc);

        let opcode = instruction >> 12;
        state.pc += 1;

        match opcode.try_into() {
            Ok(Opcode::BR) => state.conditional_branch(instruction),
            Ok(Opcode::ADD) => state.add(instruction),
            Ok(Opcode::LD) => state.load(instruction),
            Ok(Opcode::ST) => state.store(instruction),
            Ok(Opcode::JSR) => state.jump_to_subroutine(instruction),
            Ok(Opcode::AND) => state.and(instruction),
            Ok(Opcode::LDR) => state.load_base_plus_offset(instruction),
            Ok(Opcode::STR) => state.store_base_plus_offset(instruction),
            Ok(Opcode::RTI) => {
                unimplemented!()
            }
            Ok(Opcode::NOT) => state.not(instruction),
            Ok(Opcode::LDI) => state.load_indirect(instruction),
            Ok(Opcode::STI) => state.store_indirect(instruction),
            Ok(Opcode::JMP) => state.jump(instruction),
            Ok(Opcode::RES) => state.illegal_opcode(),
            Ok(Opcode::LEA) => state.load_effective_address(instruction),
            Ok(Opcode::TRAP) => state.trap(instruction),
            Err(_) => {
                panic!(
                    "Unknown Opcode: {}\nRegisters: {:?}\nPc: {:x}\nZF: {}\nNF: {}\nPF: {}\n",
                    opcode, state.registers, state.pc, state.fzero, state.fneg, state.fpos
                );
            }
        }
    }

    restore_input_buffering();
}

// Read the first two bytes to figure out the origin, then read the rest of
// the contents of the file and put it in state.memory starting from said origin.
// Every single value needs to be swapped to account for big endianness
fn load_image_file(state: &mut State, file_path: &str) -> Result<(), std::io::Error> {
    let contents = std::fs::read(file_path)?;

    let buffer: Vec<u16> = contents
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_be_bytes([a[0], a[1]]))
        .collect();

    let origin = buffer[0];

    let _ = &state.memory[origin as usize..origin as usize + buffer.len() - 1]
        .copy_from_slice(&buffer[1..]);

    Ok(())
}

fn disable_input_buffering() {
    let mut termios = Termios::from_fd(0).unwrap();
    termios.c_lflag &= !ICANON & !ECHO;
    termios::tcsetattr(0, TCSANOW, &termios).unwrap();
}

extern "C" fn handle_interrupt(_: i32) {
    restore_input_buffering();
    println!("");
    std::process::exit(2);
}

fn restore_input_buffering() {
    let mut termios = Termios::from_fd(0).unwrap();
    termios.c_lflag |= ICANON | ECHO;
    termios::tcsetattr(0, TCSANOW, &termios).unwrap();
}
