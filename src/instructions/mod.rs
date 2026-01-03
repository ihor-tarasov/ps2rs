use crate::{CPUError, CPUResult, EmotionEngine, Gpr, Instruction};

pub mod cop0;
pub mod special;

pub const SPECIAL: u32 = 0x00;
pub const SLTI: u32 = 0x0A;
pub const COP0: u32 = 0x10;

pub fn execute(cpu: &mut EmotionEngine, instruction: Instruction) -> CPUResult {
    match instruction.op() {
        SPECIAL => special::execute(cpu, instruction),
        SLTI => slti(cpu, instruction),
        COP0 => cop0::execute(cpu, instruction),
        opcode => Err(CPUError::Opcode(opcode)),
    }
}

pub fn slti(cpu: &mut EmotionEngine, instruction: Instruction) -> CPUResult {
    let rs = instruction.rs();
    let rt = instruction.rt();
    let imm = instruction.imm() as i16 as i32;
    println!("slti ${}, ${}, {imm}", Gpr::name(rt), Gpr::name(rs));
    let rs_value = cpu.gpr().read(rs) as u32 as i32;
    let result = if rs_value < imm { 1 } else { 0 };
    cpu.gpr_mut().write(rt, result);
    cpu.increment_pc();
    Ok(())
}
