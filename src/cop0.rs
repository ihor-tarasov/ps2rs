pub struct Coprocessor0 {
    registers: [u32; Self::REGISTERS_COUNT],
}

impl Coprocessor0 {
    const REGISTERS_COUNT: usize = 32;

    pub const fn new() -> Self {
        Self {
            registers: [0; Self::REGISTERS_COUNT],
        }
    }

    pub const fn mfc(&self, index: u32) -> u32 {
        self.registers[index as usize]
    }
}
