use nix::sys::signal;
use std::env;
use termios::*;

mod state;
use state::State;

mod opcode;
use opcode::Opcode;

mod disassembler;

const PC_START: u16 = 0x3000;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("lc3 [image-file1] [image-file2] ... to run object files.");
        println!("lc3 --disassemble [image-file1] [image-file2] ...  to disassemble them.");
        std::process::exit(0);
    }

    if args[1] == "--disassemble" {
        for file_path in &args[2..] {
            let buffer = load_image_file(file_path).unwrap();

            let origin = buffer[0];

            disassembler::disassemble(&buffer[1..].to_vec(), origin);
        }

        std::process::exit(0);
    } else {
        let mut state = State::new();
        for file_path in &args[1..] {
            let buffer = load_image_file(file_path).unwrap();

            // Read the first two bytes to figure out the origin, then put the
            // rest of the file in state.memory starting from said origin.
            let origin = buffer[0];

            let _ = &state.memory[origin as usize..origin as usize + buffer.len() - 1]
                .copy_from_slice(&buffer[1..]);
        }

        execute(&mut state);
    }
}

fn execute(state: &mut State) {
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

// Every single value needs to be swapped to account for big endianness
fn load_image_file(file_path: &str) -> Result<Vec<u16>, std::io::Error> {
    let contents = std::fs::read(file_path)?;

    let buffer: Vec<u16> = contents
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_be_bytes([a[0], a[1]]))
        .collect();

    Ok(buffer)
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
