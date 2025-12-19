use std::fmt;

pub enum Error {
    ReadU32(u32),
    Opcode(u32),
    Cop0(u32),
    SpecialOpcode(u32),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadU32(address) => write!(f, "Failed to read u32 by 0x{address:08X}"),
            Self::Opcode(opcode) => write!(f, "Unknown opcode 0x{opcode:02X}"),
            Self::Cop0(opcode) => write!(f, "Unknown Coprocessor0 opcode: 0x{opcode:02X}"),
            Self::SpecialOpcode(opcode) => write!(f, "Unknown special opcode 0x{opcode:02X}"),
        }
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;
