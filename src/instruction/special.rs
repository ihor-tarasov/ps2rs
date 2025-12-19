use crate::{EmotionEngine, Error, Result, instruction::Instruction};

pub fn execute(cpu: &mut EmotionEngine, instruction: Instruction) -> Result {
    match instruction.funct() {
        0x00 => sll(cpu, instruction),
        opcode => Err(Error::Special(opcode)),
    }
}

fn sll(cpu: &mut EmotionEngine, instruction: Instruction) -> Result {
    let src = instruction.rt();
    let dst = instruction.rd();
    let shift = instruction.shamt();
    log::info!("sll {{{dst}}} {{{src}}} {shift}");
    let mut value = cpu.read_gpr::<u32>(src, 0) as u64;
    value <<= shift;
    cpu.write_gpr(dst, value as i64, 0);
    Ok(())
}
