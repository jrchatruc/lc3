use std::convert::TryFrom;

#[derive(Debug)]
pub enum Opcode {
    BR,
    ADD,
    LD,
    ST,
    JSR,
    AND,
    LDR,
    STR,
    RTI,
    NOT,
    LDI,
    STI,
    JMP,
    RES,
    LEA,
    TRAP,
}

pub enum Trapcode {
    /// Get character from keyboard, not echoed into the terminal
    GETC = 0x20,
    /// Output a character
    OUT = 0x21,
    /// Output a word string
    PUTS = 0x22,
    /// Get character from keyboard, echoed into the terminal
    IN = 0x23,
    /// Output a byte string
    PUTSP = 0x24,
    /// Halt the program
    HALT = 0x25,
}

impl TryFrom<u16> for Opcode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == Opcode::BR as u16 => Ok(Opcode::BR),
            x if x == Opcode::ADD as u16 => Ok(Opcode::ADD),
            x if x == Opcode::LD as u16 => Ok(Opcode::LD),
            x if x == Opcode::ST as u16 => Ok(Opcode::ST),
            x if x == Opcode::JSR as u16 => Ok(Opcode::JSR),
            x if x == Opcode::AND as u16 => Ok(Opcode::AND),
            x if x == Opcode::LDR as u16 => Ok(Opcode::LDR),
            x if x == Opcode::STR as u16 => Ok(Opcode::STR),
            x if x == Opcode::RTI as u16 => Ok(Opcode::RTI),
            x if x == Opcode::NOT as u16 => Ok(Opcode::NOT),
            x if x == Opcode::LDI as u16 => Ok(Opcode::LDI),
            x if x == Opcode::STI as u16 => Ok(Opcode::STI),
            x if x == Opcode::JMP as u16 => Ok(Opcode::JMP),
            x if x == Opcode::RES as u16 => Ok(Opcode::RES),
            x if x == Opcode::LEA as u16 => Ok(Opcode::LEA),
            x if x == Opcode::TRAP as u16 => Ok(Opcode::TRAP),
            _ => Err(()),
        }
    }
}

impl TryFrom<u16> for Trapcode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == Trapcode::GETC as u16 => Ok(Trapcode::GETC),
            x if x == Trapcode::OUT as u16 => Ok(Trapcode::OUT),
            x if x == Trapcode::PUTS as u16 => Ok(Trapcode::PUTS),
            x if x == Trapcode::IN as u16 => Ok(Trapcode::IN),
            x if x == Trapcode::PUTSP as u16 => Ok(Trapcode::PUTSP),
            x if x == Trapcode::HALT as u16 => Ok(Trapcode::HALT),
            _ => Err(()),
        }
    }
}
