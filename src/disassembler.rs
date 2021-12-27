use super::opcode::{Opcode, Trapcode};

pub fn disassemble(code: &Vec<u16>, pc_start: u16) {
    let mut pc = pc_start;

    for instruction in code {
        print!(
            "0x{:X} {:04b} {:04b} {:04b} {:04b} ",
            pc,
            instruction >> 12,
            (instruction >> 8) & 0xF,
            (instruction >> 4) & 0xF,
            instruction & 0xF
        );
        pc += 1;

        disassemble_op(*instruction, pc);
    }
}

fn disassemble_op(instruction: u16, pc: u16) {
    let opcode = instruction >> 12;

    match opcode.try_into() {
        Ok(Opcode::BR) => conditional_branch(instruction, pc),
        Ok(Opcode::ADD) => add(instruction),
        Ok(Opcode::LD) => load(instruction, pc),
        Ok(Opcode::ST) => store(instruction, pc),
        Ok(Opcode::JSR) => jump_to_subroutine(instruction, pc),
        Ok(Opcode::AND) => and(instruction),
        Ok(Opcode::LDR) => load_base_plus_offset(instruction),
        Ok(Opcode::STR) => store_base_plus_offset(instruction),
        Ok(Opcode::RTI) => {
            unimplemented!()
        }
        Ok(Opcode::NOT) => not(instruction),
        Ok(Opcode::LDI) => load_indirect(instruction, pc),
        Ok(Opcode::STI) => store_indirect(instruction, pc),
        Ok(Opcode::JMP) => jump(instruction),
        Ok(Opcode::RES) => illegal_opcode(),
        Ok(Opcode::LEA) => load_effective_address(instruction, pc),
        Ok(Opcode::TRAP) => trap(instruction),
        Err(_) => {
            panic!("Unknown Opcode: {}", opcode);
        }
    }
}

fn conditional_branch(instruction: u16, pc: u16) {
    let n = (instruction >> 11) & 0x1;
    let z = (instruction >> 10) & 0x1;
    let p = (instruction >> 9) & 0x1;

    let offset = sign_extend(instruction & 0x01FF, 9);

    let n_character = if n == 1 { "n" } else { "" };
    let z_character = if z == 1 { "z" } else { "" };
    let p_character = if p == 1 { "p" } else { "" };

    println!(
        "BR{}{}{} 0x{:X}",
        n_character,
        z_character,
        p_character,
        pc + offset
    );
}

fn add(instruction: u16) {
    let dr = get_dr(instruction);
    let sr1 = (instruction & 0b0001_1100_0000) >> 6;

    if instruction & 0x10 == 0 {
        let sr2 = instruction & 0x0007;
        println!("ADD R{} R{} R{}", dr, sr1, sr2);
    } else {
        let imm5 = instruction & 0x001F;
        println!("ADD IMM R{} R{} 0x{:X}", dr, sr1, imm5);
    }
}

fn load(instruction: u16, pc: u16) {
    let dr = get_dr(instruction);
    let offset = sign_extend(instruction & 0x01FF, 9);

    println!("LD R{} 0x{:X}", dr, pc + offset)
}

fn store(instruction: u16, pc: u16) {
    let sr = (instruction & 0b1110_0000_0000) >> 9;
    let offset = sign_extend(instruction & 0x01FF, 9);

    println!("ST R{} 0x{:X}", sr, pc + offset);
}

fn jump_to_subroutine(instruction: u16, pc: u16) {
    if (instruction >> 11) & 0x1 == 0 {
        let base_register = (instruction >> 6) & 0x07;
        println!("JSRR R{}", base_register);
    } else {
        let offset = sign_extend(instruction & 0x07FF, 11);
        println!("JSR 0x{:X}", pc + offset);
    }
}

fn and(instruction: u16) {
    let dr = get_dr(instruction);
    let sr1 = (instruction & 0b0001_1100_0000) >> 6;

    if instruction & 0x10 == 0 {
        let sr2 = instruction & 0x0007;
        println!("AND R{} R{} R{}", dr, sr1, sr2);
    } else {
        let imm5 = instruction & 0x001F;
        println!("AND IMM R{} R{} 0x{:X}", dr, sr1, imm5);
    }
}

fn load_base_plus_offset(instruction: u16) {
    let dr = get_dr(instruction);
    let base_register = (instruction >> 6) & 0x07;
    let offset = sign_extend(instruction & 0x003F, 6);

    println!("LDR R{} R{} 0x{:X}", dr, base_register, offset);
}

fn store_base_plus_offset(instruction: u16) {
    let sr = (instruction & 0b1110_0000_0000) >> 9;
    let base_register = (instruction >> 6) & 0x07;
    let offset = sign_extend(instruction & 0x003F, 6);

    println!("STR R{} R{} 0x{:X}", sr, base_register, offset);
}

fn not(instruction: u16) {
    let dr = get_dr(instruction);
    let sr = (instruction & 0b0001_1100_0000) >> 6;

    println!("NOT R{} R{}", dr, sr);
}

fn load_indirect(instruction: u16, pc: u16) {
    let dr = get_dr(instruction);
    let offset = sign_extend(instruction & 0x01FF, 9);

    println!("LDI R{} 0x{:x}", dr, pc + offset);
}

fn store_indirect(instruction: u16, pc: u16) {
    let sr = (instruction & 0b1110_0000_0000) >> 9;
    let offset = sign_extend(instruction & 0x01FF, 9);

    println!("STI R{} 0x{:X}", sr, pc + offset);
}

fn jump(instruction: u16) {
    let base_register = (instruction >> 6) & 0x07;

    if base_register != 0b111 {
        println!("JMP R{}", base_register);
    } else {
        println!("RET");
    }
}

fn illegal_opcode() {
    panic!("Illegal opcode encountered");
}

fn load_effective_address(instruction: u16, pc: u16) {
    let dr = get_dr(instruction);
    let offset = sign_extend(instruction & 0x01FF, 9);

    println!("LEA R{} 0x{:X}", dr, pc + offset);
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
