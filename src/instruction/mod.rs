mod cop;

use crate::{EmotionEngine, Error, Result};

pub fn execute(cpu: &mut EmotionEngine, instruction: u32) -> Result {
    match instruction >> 26 {
        0x10 => cop::execute(cpu, instruction),
        opcode => Err(Error::Opcode(opcode)),
    }
}
