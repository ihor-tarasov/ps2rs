use crate::{Bus, Instruction, Result, cop0::Coprocessor0};

pub struct EmotionEngine {
    pc: u32,
    cop0: Coprocessor0,
    registers: [u8; Self::REGISTERS_COUNT << Self::REGISTER_SIZE_SHIFT],
}

impl Default for EmotionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl EmotionEngine {
    const REGISTERS_COUNT: usize = 32;
    const REGISTER_SIZE_SHIFT: usize = 4;

    const PC_START: u32 = 0xBFC0_0000;

    pub const fn new() -> Self {
        Self {
            pc: Self::PC_START,
            cop0: Coprocessor0::new(),
            registers: [0; Self::REGISTERS_COUNT << Self::REGISTER_SIZE_SHIFT],
        }
    }

    pub fn step(&mut self, bus: &mut Bus) -> Result {
        let instruction = self.read_u32(bus, self.pc)?;
        Instruction::from_u32(instruction).execute(self)?;
        self.pc += 4;
        Ok(())
    }

    pub const fn read_u32(&self, bus: &Bus, address: u32) -> Result<u32> {
        bus.read_u32(address & 0x1FFF_FFFF)
    }

    pub fn mfc0(&mut self, register: u32, cop_register: u32) {
        let value = self.cop0.mfc(cop_register);
        self.write_register_i64(register, value as i64 as i32 as i64, 0);
    }

    pub fn read_register_u32(&mut self, index: u32, offset: u32) -> u32 {
        let start = (index << Self::REGISTER_SIZE_SHIFT) + (offset << 2);
        let end_exclusive = start + 4;
        let bytes = &self.registers[(start as usize)..(end_exclusive as usize)];
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    pub fn write_register_i32(&mut self, index: u32, value: i32, offset: u32) {
        let start = (index << Self::REGISTER_SIZE_SHIFT) + (offset << 2);
        let end_exclusive = start + 4;
        (&mut self.registers[(start as usize)..(end_exclusive as usize)])
            .copy_from_slice(&value.to_le_bytes());
    }

    pub fn write_register_i64(&mut self, index: u32, value: i64, offset: u32) {
        let start = (index << Self::REGISTER_SIZE_SHIFT) + (offset << 3);
        let end_exclusive = start + 8;
        (&mut self.registers[(start as usize)..(end_exclusive as usize)])
            .copy_from_slice(&value.to_le_bytes());
    }
}
