use crate::{CPUError, CPUResult, Cop0, EmotionEngine, Gpr, Instruction};

pub const MFC0: u32 = 0x00;

pub fn execute(cpu: &mut EmotionEngine, instruction: Instruction) -> CPUResult {
    match instruction.rs() {
        MFC0 => mfc0(cpu, instruction),
        opcode => Err(CPUError::COP0Opcode(opcode)),
    }
}

pub fn mfc0(cpu: &mut EmotionEngine, instruction: Instruction) -> CPUResult {
    let rd = instruction.rd();
    let rt = instruction.rt();
    let sel = instruction.sel();
    println!("mfc0 ${}, ${}, {sel}", Gpr::name(rt), Cop0::name(rd));
    let value = cpu.cop0().read(rd, sel);
    cpu.gpr_mut().write(rt, value as i32 as i64 as u64);
    cpu.increment_pc();
    Ok(())
}
