use super::opcode::Trapcode;
use std::io::{Read, Write};

const KEYBOARD_STATUS_REGISTER: u16 = 0xFE00;
const KEYBOARD_DATA_REGISTER: u16 = 0xFE02;

#[derive(Debug)]
pub struct State {
    /// Array of registers R0 through R7
    pub registers: [u16; 8],
    /// Program counter
    pub pc: u16,
    /// Zero flag
    pub fzero: bool,
    /// Negative flag
    pub fneg: bool,
    /// Positive flag
    pub fpos: bool,
    /// Array holding the entire memory
    pub memory: [u16; u16::MAX as usize],
    /// Whether the vm is running or not
    pub running: bool,
}

impl State {
    pub fn new() -> State {
        State {
            registers: [0; 8],
            pc: 0x300,
            fzero: false,
            fneg: false,
            fpos: false,
            memory: [0; u16::MAX as usize],
            running: true,
        }
    }

    pub fn add(&mut self, instruction: u16) {
        let dr = get_dr(instruction);
        let sr1 = (instruction >> 6) & 0x07;

        if (instruction >> 5) & 0x1 == 0 {
            let sr2 = instruction & 0x07;
            self.registers[dr as usize] =
                self.registers[sr1 as usize] + self.registers[sr2 as usize];
        } else {
            let imm5 = instruction & 0x1F;
            self.registers[dr as usize] = self.registers[sr1 as usize] + sign_extend(imm5, 5);
        }

        self.update_flags(dr);
    }

    pub fn and(&mut self, instruction: u16) {
        let dr = get_dr(instruction);
        let sr1 = (instruction >> 6) & 0x07;

        if (instruction >> 5) & 0x1 == 0 {
            let sr2 = instruction & 0x07;
            self.registers[dr as usize] =
                self.registers[sr1 as usize] & self.registers[sr2 as usize];
        } else {
            let imm5 = instruction & 0x1F;
            self.registers[dr as usize] = self.registers[sr1 as usize] & sign_extend(imm5, 5);
        }

        self.update_flags(dr);
    }

    pub fn load_indirect(&mut self, instruction: u16) {
        let dr = get_dr(instruction);
        let offset = sign_extend(instruction & 0x01FF, 9);
        let address = self.mem_read(self.pc + offset);

        self.registers[dr as usize] = self.mem_read(address);
        self.update_flags(dr);
    }

    pub fn conditional_branch(&mut self, instruction: u16) {
        let n = (instruction >> 11) & 0x1;
        let z = (instruction >> 10) & 0x1;
        let p = (instruction >> 9) & 0x1;

        if (n == 1 && self.fneg) || (z == 1 && self.fzero) || (p == 1 && self.fpos) {
            let offset = sign_extend(instruction & 0x01FF, 9);
            self.pc += offset;
        }
    }

    pub fn jump(&mut self, instruction: u16) {
        let base_register = (instruction >> 6) & 0x07;

        self.pc = self.registers[base_register as usize];
    }

    pub fn jump_to_subroutine(&mut self, instruction: u16) {
        self.registers[7] = self.pc;

        if (instruction >> 11) & 0x1 == 0 {
            let base_register = (instruction >> 6) & 0x07;
            self.pc = self.registers[base_register as usize];
        } else {
            let offset = sign_extend(instruction & 0x07FF, 11);
            self.pc += offset;
        }
    }

    pub fn load(&mut self, instruction: u16) {
        let dr = get_dr(instruction);
        let offset = sign_extend(instruction & 0x01FF, 9);

        self.registers[dr as usize] = self.mem_read(self.pc + offset);
        self.update_flags(dr);
    }

    pub fn load_base_plus_offset(&mut self, instruction: u16) {
        let dr = get_dr(instruction);
        let base_register = (instruction >> 6) & 0x07;
        let offset = sign_extend(instruction & 0x3F, 6);

        self.registers[dr as usize] =
            self.mem_read(self.registers[base_register as usize] + offset);
        self.update_flags(dr);
    }

    pub fn load_effective_address(&mut self, instruction: u16) {
        let dr = get_dr(instruction);
        let offset = sign_extend(instruction & 0x01FF, 9);

        self.registers[dr as usize] = self.pc + offset;
        self.update_flags(dr);
    }

    pub fn not(&mut self, instruction: u16) {
        let dr = get_dr(instruction);
        let sr = (instruction >> 6) & 0x07;

        self.registers[dr as usize] = !self.registers[sr as usize];
        self.update_flags(dr);
    }

    pub fn store(&mut self, instruction: u16) {
        let sr = (instruction >> 9) & 0x07;
        let offset = sign_extend(instruction & 0x01FF, 9);

        self.mem_set(self.pc + offset, sr);
    }

    pub fn store_indirect(&mut self, instruction: u16) {
        let sr = (instruction >> 9) & 0x07;
        let offset = sign_extend(instruction & 0x01FF, 9);
        let address = self.mem_read(self.pc + offset);

        self.mem_set(address, sr);
    }

    pub fn store_base_plus_offset(&mut self, instruction: u16) {
        let sr = (instruction >> 9) & 0x07;
        let base_register = (instruction >> 6) & 0x07;
        let offset = sign_extend(instruction & 0x3F, 6);

        self.mem_set(
            self.registers[base_register as usize] + offset,
            self.registers[sr as usize],
        );
    }

    pub fn trap(&mut self, instruction: u16) {
        let trap_code = instruction & 0xFF;

        match trap_code.try_into() {
            Ok(Trapcode::GETC) => self.getc(),
            Ok(Trapcode::OUT) => self.out(),
            Ok(Trapcode::PUTS) => self.puts(),
            Ok(Trapcode::IN) => self.input(),
            Ok(Trapcode::PUTSP) => self.putsp(),
            Ok(Trapcode::HALT) => self.halt(),
            _ => panic!(
                "Unexpected trap code. Code: {}\nRegisters: {:?}\nPc: 0x{:x}\nZF: {}\nNF: {}\nPF: {}\n",
                trap_code, self.registers, self.pc, self.fzero, self.fneg, self.fpos
            ),
        }
    }

    pub fn illegal_opcode(&self) {
        panic!("Illegal opcode encountered")
    }

    fn getc(&mut self) {
        let input = get_char();
        self.registers[0] = input as u16;
    }

    fn out(&self) {
        let value = self.registers[0] as u8;

        print!("{}", value as char);
        let _ = std::io::stdout().flush();
    }

    fn puts(&mut self) {
        let mut index = self.registers[0];

        loop {
            let next_char = self.mem_read(index) as u8;
            if next_char == 0 {
                break;
            }

            print!("{}", next_char as char);
            index += 1;
        }
        let _ = std::io::stdout().flush();
    }

    fn input(&mut self) {
        print!("Enter a character: ");
        let _ = std::io::stdout().flush();
        let input = get_char();

        self.registers[0] = input as u16;
        print!("{}", input as char);
    }

    fn putsp(&mut self) {
        let mut index = self.registers[0];

        loop {
            let next_word = self.mem_read(index);
            let low = (next_word & 0xFF) as u8;
            let high = (next_word >> 8) as u8;

            if low == 0 {
                break;
            }
            print!("{}", low as char);

            if high == 0 {
                break;
            }
            print!("{}", high as char);

            index += 1;
        }
        let _ = std::io::stdout().flush();
    }

    fn halt(&mut self) {
        let _ = std::io::stdout().flush();
        self.running = false;
    }

    pub fn mem_read(&mut self, address: u16) -> u16 {
        // The way the keyboard status and data registers would be used normally
        // is: whenever the user presses a key, the keyboard
        // sets the status register's highest bit to one and the value of the
        // key pressed into the data register; but that's not what MY keyboard
        // will actually do when I press a button, so I have to emulate it.

        // The way we do it is the following: when the executing program wants to read
        // the status register, we check if a key has been pressed in the past; if it has, we set
        // the status register's highest bit to one and its value to the data register.
        // Otherwise we just set the status register to zero.
        if address == KEYBOARD_STATUS_REGISTER {
            if check_key() {
                self.memory[KEYBOARD_STATUS_REGISTER as usize] = 1 << 15;
                self.memory[KEYBOARD_DATA_REGISTER as usize] = get_char() as u16;
            } else {
                self.memory[KEYBOARD_STATUS_REGISTER as usize] = 0;
            }
        }

        return self.memory[address as usize];
    }

    fn mem_set(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }

    fn update_flags(&mut self, register: u16) {
        let value = self.registers[register as usize];

        // TODO?: maybe change this to an enum idk.
        if value == 0 {
            self.fneg = false;
            self.fpos = false;
            self.fzero = true;
        } else if (value >> 15) == 1 {
            self.fzero = false;
            self.fpos = false;
            self.fneg = true;
        } else {
            self.fzero = false;
            self.fneg = false;
            self.fpos = true;
        }
    }
}

fn sign_extend(value: u16, bit_count: u16) -> u16 {
    if ((value >> (bit_count - 1)) & 1) == 1 {
        value | (0xFFFF << bit_count)
    } else {
        value
    }
}

fn check_key() -> bool {
    let mut readfds = nix::sys::select::FdSet::new();
    readfds.insert(0);
    let mut timeout: nix::sys::time::TimeVal = nix::sys::time::TimeValLike::seconds(0);

    return nix::sys::select::select(1, &mut readfds, None, None, &mut timeout).unwrap() != 0;
}

fn get_dr(instruction: u16) -> u16 {
    return (instruction >> 9) & 0x7;
}

fn get_char() -> u8 {
    let input = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8);

    match input {
        Some(value) => value,
        _ => panic!("Error reading from stdin"),
    }
}
