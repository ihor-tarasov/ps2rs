use crate::{Bus, Instruction, Result, cop0::Coprocessor0};

pub struct EmotionEngine {
    pc: u32,
    cop0: Coprocessor0,
    gpr: [u128; Self::REGISTERS_COUNT],
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
        Self {
            pc: Self::PC_START,
            cop0: Coprocessor0::new(),
            gpr: [0; Self::REGISTERS_COUNT],
        }
    }

    pub fn step(&mut self, bus: &mut Bus) -> Result {
        let instruction = Instruction::from_le_bytes(self.read4(bus, self.pc)?);
        instruction.execute(self)?;
        self.pc += 4;
        Ok(())
    }

    pub const fn read4(&self, bus: &Bus, address: u32) -> Result<[u8; 4]> {
        bus.read4(address & 0x1FFF_FFFF)
    }

    pub fn mfc0(&mut self, register: u8, cop_register: u8) {
        let value = self.cop0.mfc(cop_register);
        self.write_gpr(register, value as i64 as i32 as i64, 0);
    }

    pub fn read_gpr<T>(&mut self, index: u8, offset: u8) -> T
    where
        T: bytemuck::AnyBitPattern,
    {
        bytemuck::cast_slice::<_, T>(&[self.gpr[index as usize]])[offset as usize]
    }

    pub fn write_gpr<T>(&mut self, index: u8, value: T, offset: u8)
    where
        T: bytemuck::NoUninit + bytemuck::AnyBitPattern,
    {
        bytemuck::cast_slice_mut::<_, T>(&mut [self.gpr[index as usize]])[offset as usize] = value;
    }
}
