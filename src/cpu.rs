use crate::{Bus, Coprocessor0, Instruction, Result};

pub struct EmotionEngine {
    pc: u32,
    cop0: Coprocessor0,
    gpr: [u64; Self::REGISTERS_COUNT],
    branch: bool,
    delay: u8,
    next_pc: u32,
}

impl Default for EmotionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl EmotionEngine {
    const REGISTERS_COUNT: usize = 32;

    const PC_START: u32 = 0xBFC0_0000;

    pub const fn new() -> Self {
        let mut cop0 = Coprocessor0::new();
        // The PS2 BIOS reads this register as the very first instruction:
        //   mfc0 $at, $15
        //
        // If this value does not identify an Emotion Engine (R5900),
        // the BIOS will assume an unsupported CPU and stop execution.
        //
        // 0x2E20 is the expected implementation/revision value for the EE.
        cop0.mtc(Coprocessor0::PRID, 0x0000_2E20);
        Self {
            pc: Self::PC_START,
            cop0,
            gpr: [0; Self::REGISTERS_COUNT],
            branch: false,
            delay: 0,
            next_pc: 0,
        }
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn step(&mut self, bus: &mut Bus) -> Result {
        if self.delay != 0 {
            self.delay -= 1;
        } else if self.branch {
            self.pc = self.next_pc;
            self.branch = false;
        }
        let instruction = Instruction::from_le_bytes(self.read4(bus, self.pc)?);
        instruction.execute(bus, self)?;
        self.cop0.increment_count();
        self.pc += 4;
        Ok(())
    }

    pub fn read4(&self, bus: &Bus, address: u32) -> Result<[u8; 4]> {
        bus.read4(address & 0x1FFF_FFFF)
    }

    pub fn write_u32(&mut self, bus: &mut Bus, address: u32, value: u32) {
        if address & 0b11 != 0 {
            panic!("Unaligned write of u32 to bus");
        }
        bus.write_u32(address, value);
    }

    pub fn branch(&mut self, condition: bool, address: u32) {
        if condition {
            self.branch = true;
            self.next_pc = address;
            self.delay = 1;
        }
    }

    pub fn jp(&mut self, address: u32) {
        self.branch = true;
        self.next_pc = address;
        self.delay = 1;
    }

    pub fn mfc0(&mut self, register: u8, cop_register: u8) {
        let value = self.cop0.mfc(cop_register);
        self.write_gpr_u32(register, value);
    }

    pub fn mtc0(&mut self, register: u8, cop_register: u8) {
        let value = self.read_gpr_u32(register);
        self.cop0.mtc(cop_register, value);
    }

    pub const fn read_gpr_u32(&mut self, index: u8) -> u32 {
        self.gpr[index as usize] as u32
    }

    pub const fn read_gpr_i32(&mut self, index: u8) -> i32 {
        self.gpr[index as usize] as i32
    }

    pub const fn read_gpr_u64(&mut self, index: u8) -> u64 {
        self.gpr[index as usize]
    }

    pub const fn read_gpr_i64(&mut self, index: u8) -> i64 {
        self.gpr[index as usize] as i64
    }

    pub const fn write_gpr_u32(&mut self, index: u8, value: u32) {
        if index != 0 {
            self.gpr[index as usize] = value as u64;
        }
    }

    pub const fn write_gpr_i32(&mut self, index: u8, value: i32) {
        if index != 0 {
            self.gpr[index as usize] = value as i64 as u64;
        }
    }

    pub const fn write_gpr_u64(&mut self, index: u8, value: u64) {
        if index != 0 {
            self.gpr[index as usize] = value;
        }
    }

    pub const fn write_gpr_i64(&mut self, index: u8, value: i64) {
        if index != 0 {
            self.gpr[index as usize] = value as u64;
        }
    }

    pub const GPR_NAMES: [&'static str; 32] = [
        "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
        "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp",
        "fp", "ra",
    ];
}
