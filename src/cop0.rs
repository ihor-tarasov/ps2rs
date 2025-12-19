/// COP0 — System Control Coprocessor (MIPS R5900 / PS2 EE)
pub struct Coprocessor0 {
    registers: [u32; Self::REGISTERS_COUNT],
}

impl Coprocessor0 {
    const REGISTERS_COUNT: usize = 32;

    /// TLB Index Register
    pub const INDEX: u8 = 0;

    /// TLB Random Register
    pub const RANDOM: u8 = 1;

    /// TLB Entry Low Register (even page)
    pub const ENTRY_LO0: u8 = 2;

    /// TLB Entry Low Register (odd page)
    pub const ENTRY_LO1: u8 = 3;

    /// TLB Context Register
    pub const CONTEXT: u8 = 4;

    /// TLB Page Mask Register
    pub const PAGE_MASK: u8 = 5;

    /// TLB Wired Register
    pub const WIRED: u8 = 6;

    /// Reserved / unused
    pub const UNUSED_7: u8 = 7;

    /// Bad Virtual Address Register
    pub const BAD_VADDR: u8 = 8;

    /// Count Register (cycle counter)
    pub const COUNT: u8 = 9;

    /// TLB Entry High Register (VPN / ASID)
    pub const ENTRY_HI: u8 = 10;

    /// Compare Register (timer interrupt)
    pub const COMPARE: u8 = 11;

    /// Processor Status Register
    pub const STATUS: u8 = 12;

    /// Exception Cause Register
    pub const CAUSE: u8 = 13;

    /// Exception Program Counter
    pub const EPC: u8 = 14;

    /// Processor Identification Register
    pub const PRID: u8 = 15;

    /// Configuration Register
    pub const CONFIG: u8 = 16;

    /// Reserved
    pub const UNUSED_17: u8 = 17;

    /// Watchpoint Low Register
    pub const WATCH_LO: u8 = 18;

    /// Watchpoint High Register
    pub const WATCH_HI: u8 = 19;

    /// Extended TLB Context Register
    pub const XCONTEXT: u8 = 20;

    /// Reserved
    pub const UNUSED_21: u8 = 21;
    pub const UNUSED_22: u8 = 22;
    pub const UNUSED_23: u8 = 23;

    /// Debug Register
    pub const DEBUG: u8 = 24;

    /// Performance Counter Register
    pub const PERF: u8 = 25;

    /// ECC Error Register
    pub const ECC: u8 = 26;

    /// Cache Error Register
    pub const CACHE_ERR: u8 = 27;

    /// Cache Tag Low Register
    pub const TAG_LO: u8 = 28;

    /// Cache Tag High Register
    pub const TAG_HI: u8 = 29;

    /// Error Exception Program Counter
    pub const ERROR_EPC: u8 = 30;

    /// Reserved
    pub const UNUSED_31: u8 = 31;

    pub const fn new() -> Self {
        Self {
            registers: [0; Self::REGISTERS_COUNT],
        }
    }

    pub const fn mfc(&self, index: u8) -> u32 {
        self.registers[index as usize]
    }

    pub const fn mtc(&mut self, index: u8, value: u32) {
        self.registers[index as usize] = value;
    }
}
