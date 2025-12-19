mod cop0;
mod special;

use crate::{EmotionEngine, Error, Result};

#[derive(Clone, Copy)]
pub struct Instruction(u32);

impl Instruction {
    pub const fn from_u32(v: u32) -> Self {
        Self(v)
    }

    pub const fn opcode(self) -> u32 {
        self.0 >> 26
    }

    pub const fn special_opcode(self) -> u32 {
        self.0 & 0x3F
    }

    pub const fn rs(self) -> u32 {
        (self.0 >> 21) & 0x1F
    }

    pub const fn rt(self) -> u32 {
        (self.0 >> 16) & 0x1F
    }

    pub const fn rd(self) -> u32 {
        (self.0 >> 11) & 0x1F
    }

    pub const fn shamt(self) -> u32 {
        (self.0 >> 6) & 0x1F
    }

    pub fn execute(self, cpu: &mut EmotionEngine) -> Result {
        match self.opcode() {
            0x00 => special::execute(cpu, self),
            0x10 => cop0::execute(cpu, self),
            opcode => Err(Error::Opcode(opcode)),
        }
    }
}
