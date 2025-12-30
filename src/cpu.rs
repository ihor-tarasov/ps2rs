use std::fmt;

use crate::{Bus, BusError};

pub enum CPUError {
    Bus(BusError),
    UnknownInstruction(u32),
}

impl From<BusError> for CPUError {
    fn from(value: BusError) -> Self {
        Self::Bus(value)
    }
}

impl fmt::Debug for CPUError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bus(bus) => write!(f, "Bus error: {bus:?}"),
            Self::UnknownInstruction(instruction) => {
                write!(f, "Unknown instruction 0x{instruction:08X}")
            }
        }
    }
}

pub type CPUResult<T = ()> = Result<T, CPUError>;

pub struct EmotionEngine {
    pc: u32,
}

impl EmotionEngine {
    const RESET_PC: u32 = 0xBFC0_0000;

    pub const fn new() -> Self {
        Self { pc: Self::RESET_PC }
    }

    pub fn step(&mut self, bus: &mut Bus) -> CPUResult {
        let instruction = bus.read_u32(self.pc)?;
        Err(CPUError::UnknownInstruction(instruction))
    }
}
