use std::fmt;

use crate::{Bus, BusError, Cop0, Gpr, Instruction, instructions};

pub enum CPUError {
    Bus(BusError),
    Opcode(u32),
    SpecialOpcode(u32),
    COP0Opcode(u32),
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
            Self::Opcode(opcode) => {
                write!(f, "Unknown opcode 0x{opcode:02X}")
            }
            Self::SpecialOpcode(opcode) => {
                write!(f, "Unknown special opcode 0x{opcode:02X}")
            }
            Self::COP0Opcode(opcode) => {
                write!(f, "Unknown COP0 opcode 0x{opcode:02X}")
            }
        }
    }
}

pub type CPUResult<T = ()> = Result<T, CPUError>;

pub struct EmotionEngine {
    pc: u32,
    gpr: Gpr,
    cop0: Cop0,
}

impl EmotionEngine {
    const RESET_PC: u32 = 0xBFC0_0000;

    pub const fn new() -> Self {
        Self {
            pc: Self::RESET_PC,
            gpr: Gpr::new(),
            cop0: Cop0::new(),
        }
    }

    pub const fn gpr(&self) -> &Gpr {
        &self.gpr
    }

    pub const fn gpr_mut(&mut self) -> &mut Gpr {
        &mut self.gpr
    }

    pub const fn cop0(&self) -> &Cop0 {
        &self.cop0
    }

    pub const fn cop0_mut(&mut self) -> &mut Cop0 {
        &mut self.cop0
    }

    pub const fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(4);
    }

    pub fn step(&mut self, bus: &mut Bus) -> CPUResult {
        let instruction = Instruction::new(bus.read_u32(self.pc)?);
        instructions::execute(self, instruction)
    }
}
