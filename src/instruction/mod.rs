mod cop0;
mod special;

use crate::{EmotionEngine, Error, Result};

#[macro_export]
macro_rules! trace_asm {
    () => {
        #[cfg(feature = "trace_asm")]
        println!()
    };
    ($($arg:tt)*) => {{
        #[cfg(feature = "trace_asm")]
        println!($($arg)*);
    }};
}

/*
        24       16        8        0
         |        |        |        |
 +--------+--------+--------+--------+
     3        2        1        0
 +--------+--------+--------+--------+
  ||||||++ +++||||| +++++||| ||++++++
  opcode  rs    rt   rd   shamt funct
 +--------+--------+--------+--------+
         | |||||||| |||||||| ||||||||
                  target
 +--------+--------+--------+--------+
                    |||||||| ||||||||
                        immediate
*/
#[derive(Clone, Copy)]
pub struct Instruction([u8; 4]);

impl Instruction {
    pub const fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }

    pub fn is_nop(self) -> bool {
        self.0 == [0; 4]
    }

    pub const fn opcode(self) -> u8 {
        self.0[3] >> 2
    }

    pub const fn funct(self) -> u8 {
        self.0[0] & 0b11_1111
    }

    pub const fn rs(self) -> u8 {
        (self.0[2] >> 5) | ((self.0[3] & 0b11) << 3)
    }

    pub const fn rt(self) -> u8 {
        self.0[2] & 0b11111
    }

    pub const fn rd(self) -> u8 {
        self.0[1] >> 3
    }

    pub const fn shamt(self) -> u8 {
        (self.0[0] >> 6) | ((self.0[1] & 0b111) << 2)
    }

    pub const fn target(self) -> u32 {
        (self.0[0] as u32)
            | ((self.0[1] as u32) << 8)
            | ((self.0[2] as u32) << 16)
            | (((self.0[3] & 0b1) as u32) << 24)
    }

    pub const fn immediate(self) -> u16 {
        (self.0[0] as u16) | ((self.0[1] as u16) << 8)
    }

    pub fn execute(self, cpu: &mut EmotionEngine) -> Result {
        match self.opcode() {
            0x00 => special::execute(cpu, self),
            0x05 => self.bne(cpu),
            0x0A => self.slti(cpu),
            0x0D => self.ori(cpu),
            0x0F => self.lui(cpu),
            0x10 => cop0::execute(cpu, self),
            opcode => Err(Error::Opcode(opcode)),
        }
    }

    pub const fn sign_extend(value: u16) -> i32 {
        value as i16 as i32
    }

    fn bne(self, cpu: &mut EmotionEngine) -> Result {
        let imm = self.immediate();
        let rs = self.rs();
        let rt = self.rt();
        let a = cpu.read_gpr::<u32>(rs, 0);
        let b = cpu.read_gpr::<u32>(rt, 0);
        trace_asm!("bne ${}, ${}, {imm}", gpr_name(rs), gpr_name(rt));
        cpu.branch(
            a != b,
            cpu.pc()
                .wrapping_add(4)
                .wrapping_add((Self::sign_extend(imm) << 2) as u32),
        );
        Ok(())
    }

    fn slti(self, cpu: &mut EmotionEngine) -> Result {
        let imm = self.immediate() as i16 as i32;
        let dst = self.rt();
        let src = self.rs();
        trace_asm!("slti ${}, ${}, {imm}", gpr_name(dst), gpr_name(src));
        let value = cpu.read_gpr::<i32>(src, 0);
        let result = if value < imm { 1 } else { 0 };
        cpu.write_gpr(dst, result as i64, 0);
        Ok(())
    }

    fn ori(self, cpu: &mut EmotionEngine) -> Result {
        let imm = self.immediate();
        let dst = self.rt();
        let src = self.rs();
        trace_asm!("ori ${}, ${}, ${imm:04X}", gpr_name(dst), gpr_name(src));
        let result = cpu.read_gpr::<u32>(src, 0) | imm as u32;
        cpu.write_gpr::<u32>(dst, result, 0);
        Ok(())
    }

    fn lui(self, cpu: &mut EmotionEngine) -> Result {
        let imm = self.immediate();
        let dst = self.rt();

        trace_asm!("lui ${}, ${imm:04X}", gpr_name(dst));

        // LUI places imm in bits 31..16, lower 16 bits are zero.
        // The 32-bit result is then sign-extended when written to the GPR.
        let value = ((imm as u32) << 16) as i32;

        cpu.write_gpr::<i32>(dst, value, 0);
        Ok(())
    }
}

fn gpr_name(index: u8) -> &'static str {
    EmotionEngine::GPR_NAMES[index as usize]
}
