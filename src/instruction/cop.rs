use crate::{EmotionEngine, Error, Result};

pub fn execute(cpu: &mut EmotionEngine, instruction: u32) -> Result {
    let opcode = (instruction >> 21) & 0x1F;
    let cop_id = (instruction >> 26) & 0b11;
    match (opcode, cop_id) {
        (0x00, 0) => mfc(cpu, instruction, cop_id),
        _ => Err(Error::Cop {
            cop_id: cop_id as u8,
            opcode: opcode as u8,
        }),
    }
}

fn mfc(cpu: &mut EmotionEngine, instruction: u32, cop_id: u32) -> Result {
    let cpu_register = (instruction >> 16) & 0x1F;
    let cop_register = (instruction >> 11) & 0x1F;
    log::trace!("mfc{cop_id} {cpu_register} {cop_register}");
    cpu.mfc(cop_id, cpu_register, cop_register)
}
