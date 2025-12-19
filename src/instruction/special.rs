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
    if instruction.is_nop() {
        trace_asm!("nop");
        return Ok(());
    }

    let src = instruction.rt();
    let dst = instruction.rd();
    let shift = instruction.shamt();
    trace_asm!("sll ${}, ${}, {shift}", gpr_name(dst), gpr_name(src));
    let mut value = cpu.read_gpr_u32(src) as u64;
    value <<= shift;
    cpu.write_gpr_i64(dst, value as i64);
    Ok(())
}

fn jr(cpu: &mut EmotionEngine, instruction: Instruction) -> Result {
    let rs = instruction.rs();
    trace_asm!("jr ${}", gpr_name(rs));
    let address = cpu.read_gpr_u32(rs);
    // TODO: Check alignment of address
    cpu.jp(address);
    Ok(())
}
