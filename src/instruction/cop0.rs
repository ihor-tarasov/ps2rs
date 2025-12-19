use crate::{EmotionEngine, Error, Result, instruction::Instruction, trace_asm};

pub fn execute(cpu: &mut EmotionEngine, instruction: Instruction) -> Result {
    let opcode = instruction.rs();
    match opcode {
        0x00 => mfc0(cpu, instruction),
        _ => Err(Error::Cop0(opcode)),
    }
}

fn mfc0(cpu: &mut EmotionEngine, instruction: Instruction) -> Result {
    let cpu_register = instruction.rt();
    let cop_register = instruction.rd();
    trace_asm!(
        "mfc0 ${}, ${cop_register}",
        Instruction::gpr_name(cpu_register),
    );
    cpu.mfc0(cpu_register, cop_register);
    Ok(())
}
