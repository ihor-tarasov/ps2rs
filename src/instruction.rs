use core::fmt;

#[derive(Clone, Copy)]
pub struct Instruction(u32);

impl Instruction {
    #[inline(always)]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Primary opcode (bits 26..=31)
    #[inline(always)]
    pub const fn op(self) -> u32 {
        (self.0 >> 26) & 0x3F
    }

    #[inline(always)]
    pub const fn nop(self) -> bool {
        self.0 == 0
    }

    /// rs (bits 21..=25)
    #[inline(always)]
    pub const fn rs(self) -> u32 {
        (self.0 >> 21) & 0x1F
    }

    /// rt (bits 16..=20)
    #[inline(always)]
    pub const fn rt(self) -> u32 {
        (self.0 >> 16) & 0x1F
    }

    /// rd (bits 11..=15)
    #[inline(always)]
    pub const fn rd(self) -> u32 {
        (self.0 >> 11) & 0x1F
    }

    /// shamt (bits 6..=10)
    #[inline(always)]
    pub const fn shamt(self) -> u32 {
        (self.0 >> 6) & 0x1F
    }

    /// funct (bits 0..=5) for SPECIAL (op=0)
    #[inline(always)]
    pub const fn funct(self) -> u32 {
        self.0 & 0x3F
    }

    /// Immediate (bits 0..=15)
    #[inline(always)]
    pub const fn imm(self) -> u32 {
        self.0 & 0xFFFF
    }

    /// Jump target (bits 0..=25)
    #[inline(always)]
    pub const fn target(self) -> u32 {
        self.0 & 0x03FF_FFFF
    }

    /// sel (bits 0..=2)
    #[inline(always)]
    pub const fn sel(self) -> u32 {
        self.0 & 0b111
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:08X}", self.0)
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instruction({})", self.0)
    }
}
