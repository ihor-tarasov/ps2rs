use crate::{EmotionEngine, Error, Result, instruction::Instruction};

pub fn execute(cpu: &mut EmotionEngine, instruction: Instruction) -> Result {
    match instruction.special_opcode() {
        0x00 => sll(cpu, instruction),
        opcode => Err(Error::SpecialOpcode(opcode)),
    }
}

fn sll(cpu: &mut EmotionEngine, instruction: Instruction) -> Result {
    let src = instruction.rt();
    let dst = instruction.rd();
    let shift = instruction.shamt();
    log::info!("sll {{{dst}}} {{{src}}} {shift}");
    let mut value = cpu.read_register_u32(src, 0) as u64;
    value <<= shift;
    cpu.write_register_i64(dst, value as i64, 0);
    Ok(())
}
