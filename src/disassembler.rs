use super::opcode::{Opcode, Trapcode};
use super::state::State;

// TODO: Refactor this, the disassembler code receives the state of execution as input, which is a bit weird
// as it intermingles it with the code in `main.rs`.

pub fn disassemble(instruction: u16, state: &State) {
    let opcode = instruction >> 12;

    print!("0x{:X} ", state.pc);

    match opcode.try_into() {
        Ok(Opcode::BR) => conditional_branch(instruction, state),
        Ok(Opcode::ADD) => add(instruction, state),
        Ok(Opcode::LD) => load(instruction, state),
        Ok(Opcode::ST) => store(instruction, state),
        Ok(Opcode::JSR) => jump_to_subroutine(instruction, state),
        Ok(Opcode::AND) => and(instruction, state),
        Ok(Opcode::LDR) => load_base_plus_offset(instruction, state),
        Ok(Opcode::STR) => store_base_plus_offset(instruction, state),
        Ok(Opcode::RTI) => {
            unimplemented!()
        }
        Ok(Opcode::NOT) => not(instruction),
        Ok(Opcode::LDI) => load_indirect(instruction, state),
        Ok(Opcode::STI) => store_indirect(instruction, state),
        Ok(Opcode::JMP) => jump(instruction, state),
        Ok(Opcode::RES) => illegal_opcode(),
        Ok(Opcode::LEA) => load_effective_address(instruction, state),
        Ok(Opcode::TRAP) => trap(instruction),
        Err(_) => {
            panic!("Unknown Opcode: {}", opcode);
        }
    }
}

fn conditional_branch(instruction: u16, state: &State) {
    let n = (instruction >> 11) & 0x1;
    let z = (instruction >> 10) & 0x1;
    let p = (instruction >> 9) & 0x1;

    let offset = sign_extend(instruction & 0x01FF, 9);

    let n_character = if n == 1 { "n" } else { "" };
    let z_character = if z == 1 { "z" } else { "" };
    let p_character = if p == 1 { "p" } else { "" };

    println!(
        "BR{}{}{} {}",
        n_character,
        z_character,
        p_character,
        state.pc + 1 + offset
    );
}

fn add(instruction: u16, state: &State) {
    let dr = get_dr(instruction);
    let sr1 = (instruction & 0b0001_1100_0000) >> 6;

    if instruction & 0x10 == 0 {
        let sr2 = instruction & 0x0007;
        println!(
            "ADD {} {} {}",
            dr, state.registers[sr1 as usize], state.registers[sr2 as usize]
        );
    } else {
        let imm5 = instruction & 0x001F;
        println!("ADD {} {} {}", dr, state.registers[sr1 as usize], imm5);
    }
}

fn load(instruction: u16, state: &State) {
    let dr = get_dr(instruction);
    let offset = sign_extend(instruction & 0x01FF, 9);

    println!("LD {} 0x{:x}", dr, state.pc + 1 + offset)
}

fn store(instruction: u16, state: &State) {
    let sr = (instruction & 0b1110_0000_0000) >> 9;
    let offset = sign_extend(instruction & 0x01FF, 9);

    println!("ST {} 0x{:x}", sr, state.pc + 1 + offset);
}

fn jump_to_subroutine(instruction: u16, state: &State) {
    if (instruction >> 11) & 0x1 == 0 {
        let base_register = (instruction >> 6) & 0x07;
        println!("JSRR {}", state.registers[base_register as usize]);
    } else {
        let offset = sign_extend(instruction & 0x07FF, 11);
        println!("JSR {}", state.pc + 1 + offset);
    }
}

fn and(instruction: u16, state: &State) {
    let dr = get_dr(instruction);
    let sr1 = (instruction & 0b0001_1100_0000) >> 6;

    if instruction & 0x10 == 0 {
        let sr2 = instruction & 0x0007;
        println!(
            "AND {} {} {}",
            dr, state.registers[sr1 as usize], state.registers[sr2 as usize]
        );
    } else {
        let imm5 = instruction & 0x001F;
        println!(
            "AND {} {} {}",
            dr, state.registers[sr1 as usize], state.registers[imm5 as usize]
        );
    }
}

fn load_base_plus_offset(instruction: u16, state: &State) {
    let dr = get_dr(instruction);
    let base_register = (instruction >> 6) & 0x07;
    let offset = sign_extend(instruction & 0x003F, 6);

    println!(
        "LDR {} {}",
        dr,
        state.registers[base_register as usize] + offset
    );
}

fn store_base_plus_offset(instruction: u16, state: &State) {
    let sr = (instruction & 0b1110_0000_0000) >> 9;
    let base_register = (instruction >> 6) & 0x07;
    let offset = sign_extend(instruction & 0x003F, 6);

    println!(
        "STR {} {} {}",
        sr, state.registers[base_register as usize], offset
    );
}

fn not(instruction: u16) {
    let dr = get_dr(instruction);
    let sr = (instruction & 0b0001_1100_0000) >> 6;

    println!("NOT {} {}", dr, sr);
}

fn load_indirect(instruction: u16, state: &State) {
    let dr = get_dr(instruction);
    let offset = sign_extend(instruction & 0x01FF, 9);

    println!("LDI {} 0x{:x}", dr, state.pc + 1 + offset);
}

fn store_indirect(instruction: u16, state: &State) {
    let sr = (instruction & 0b1110_0000_0000) >> 9;
    let offset = sign_extend(instruction & 0x01FF, 9);

    println!("STI {} 0x{:x}", sr, state.pc + 1 + offset);
}

fn jump(instruction: u16, state: &State) {
    let base_register = (instruction >> 6) & 0x07;

    if base_register != 0b111 {
        println!("JMP {}", state.registers[base_register as usize]);
    } else {
        println!("RET");
    }
}

fn illegal_opcode() {
    panic!("Illegal opcode encountered");
}

fn load_effective_address(instruction: u16, state: &State) {
    let dr = get_dr(instruction);
    let offset = sign_extend(instruction & 0x01FF, 9);

    println!("LEA {} 0x{:x}", dr, state.pc + 1 + offset);
}

fn trap(instruction: u16) {
    let trap_code = instruction & 0xFF;

    let trap_vector = match trap_code.try_into() {
        Ok(Trapcode::GETC) => "GETC",
        Ok(Trapcode::OUT) => "OUT",
        Ok(Trapcode::PUTS) => "PUTS",
        Ok(Trapcode::IN) => "IN",
        Ok(Trapcode::PUTSP) => "PUTSP",
        Ok(Trapcode::HALT) => "HALT",
        _ => "",
    };

    if trap_vector != "" {
        println!("TRAP {}", trap_vector);
    } else {
        println!("TRAP {}", trap_code);
    }
}

fn sign_extend(value: u16, bit_count: u16) -> u16 {
    if (value >> (bit_count - 1) & 1) == 1 {
        value | (0xFFFF << bit_count)
    } else {
        value
    }
}

fn get_dr(instruction: u16) -> u16 {
    return (instruction & 0b1110_0000_0000) >> 9;
}
