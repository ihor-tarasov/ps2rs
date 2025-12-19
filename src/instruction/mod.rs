mod cop;
mod special;

use crate::{EmotionEngine, Error, Result};

pub fn execute(cpu: &mut EmotionEngine, instruction: u32) -> Result {
    match instruction >> 26 {
        0x00 => special::execute(cpu, instruction),
        0x10 => cop::execute(cpu, instruction),
        opcode => Err(Error::Opcode(opcode)),
    }
}
