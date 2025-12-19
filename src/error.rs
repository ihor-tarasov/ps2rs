use std::fmt;

pub enum Error {
    ReadU32(u32),
    Opcode(u32),
    Cop { cop_id: u8, opcode: u8 },
    MfcCopId(u8),
    SpecialOpcode(u8),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadU32(address) => write!(f, "Failed to read u32 by 0x{address:08X}"),
            Self::Opcode(opcode) => write!(f, "Unknown opcode 0x{opcode:02X}"),
            Self::Cop { cop_id, opcode } => write!(
                f,
                "Unknown cop instruction, cop id: {cop_id}, opcode: 0x{opcode:02X}"
            ),
            Self::MfcCopId(cop_id) => write!(f, "Unknown mfc cop id: {cop_id}"),
            Self::SpecialOpcode(opcode) => write!(f, "Unknown special opcode 0x{opcode:02X}"),
        }
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;
