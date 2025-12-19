use crate::{EmotionEngine, Error, Result};

pub fn execute(cpu: &mut EmotionEngine, instruction: u32) -> Result {
    match instruction & 0x3F {
        0x00 => sll(cpu, instruction),
        opcode => Err(Error::SpecialOpcode(opcode as u8)),
    }
}

fn sll(cpu: &mut EmotionEngine, instruction: u32) -> Result {
    let src = (instruction >> 16) & 0x1F;
    let dst = (instruction >> 11) & 0x1F;
    let shift = (instruction >> 6) & 0x1F;
    log::info!("sll {{{dst}}} {{{src}}} {shift}");
    let mut value = cpu.read_register_u32(src, 0) as u64;
    value <<= shift;
    cpu.write_register_i64(dst, value as i64, 0);
    Ok(())
}
