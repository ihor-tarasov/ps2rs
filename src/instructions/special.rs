use crate::{CPUError, CPUResult, EmotionEngine, Gpr, Instruction};

pub const SLL: u32 = 0x00;

pub fn execute(cpu: &mut EmotionEngine, instruction: Instruction) -> CPUResult {
    match instruction.funct() {
        SLL => sll(cpu, instruction),
        opcode => Err(CPUError::SpecialOpcode(opcode)),
    }
}

pub fn sll(cpu: &mut EmotionEngine, instruction: Instruction) -> CPUResult {
    if instruction.nop() {
        println!("nop");
        cpu.increment_pc();
        return Ok(());
    }
    let rd = instruction.rd();
    let rt = instruction.rt();
    let sa = instruction.shamt();
    println!("sll ${}, ${}, {sa}", Gpr::name(rd), Gpr::name(rt));
    let value = (cpu.gpr().read(rt) as u32) << sa;
    cpu.gpr_mut().write(rd, value as i32 as i64 as u64);
    cpu.increment_pc();
    Ok(())
}
