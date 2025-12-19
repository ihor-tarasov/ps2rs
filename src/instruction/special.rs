use crate::{
    EmotionEngine, Error, Result,
    instruction::{Instruction, gpr_name},
    trace_asm,
};

pub fn execute(cpu: &mut EmotionEngine, instruction: Instruction) -> Result {
    match instruction.funct() {
        0x00 => sll(cpu, instruction),
        0x08 => jr(cpu, instruction),
        opcode => Err(Error::Special(opcode)),
    }
}

fn sll(cpu: &mut EmotionEngine, instruction: Instruction) -> Result {
    let src = instruction.rt();
    let dst = instruction.rd();
    let shift = instruction.shamt();
    trace_asm!("sll ${}, ${}, {shift}", gpr_name(dst), gpr_name(src));
    let mut value = cpu.read_gpr::<u32>(src, 0) as u64;
    value <<= shift;
    cpu.write_gpr(dst, value as i64, 0);
    Ok(())
}

fn jr(cpu: &mut EmotionEngine, instruction: Instruction) -> Result {
    let rs = instruction.rs();
    trace_asm!("jr ${}", gpr_name(rs));
    let address = cpu.read_gpr::<u32>(rs, 0);
    // TODO: Check alignment of address
    cpu.jp(address);
    Ok(())
}
