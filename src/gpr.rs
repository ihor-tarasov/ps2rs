const COUNT: usize = 32;

pub struct Gpr([u64; COUNT]);

impl Gpr {
    pub const ZERO: u32 = 0;
    pub const AT: u32 = 1;

    pub const V0: u32 = 2;
    pub const V1: u32 = 3;

    pub const A0: u32 = 4;
    pub const A1: u32 = 5;
    pub const A2: u32 = 6;
    pub const A3: u32 = 7;

    pub const T0: u32 = 8;
    pub const T1: u32 = 9;
    pub const T2: u32 = 10;
    pub const T3: u32 = 11;
    pub const T4: u32 = 12;
    pub const T5: u32 = 13;
    pub const T6: u32 = 14;
    pub const T7: u32 = 15;

    pub const S0: u32 = 16;
    pub const S1: u32 = 17;
    pub const S2: u32 = 18;
    pub const S3: u32 = 19;
    pub const S4: u32 = 20;
    pub const S5: u32 = 21;
    pub const S6: u32 = 22;
    pub const S7: u32 = 23;

    pub const T8: u32 = 24;
    pub const T9: u32 = 25;

    pub const K0: u32 = 26;
    pub const K1: u32 = 27;

    pub const GP: u32 = 28;
    pub const SP: u32 = 29;

    pub const FP: u32 = 30;
    pub const S8: u32 = 30;

    pub const RA: u32 = 31;

    pub const fn new() -> Self {
        Self([0; COUNT])
    }

    #[inline(always)]
    pub const fn read(&self, index: u32) -> u64 {
        self.0[index as usize]
    }

    #[inline(always)]
    pub const fn write(&mut self, index: u32, value: u64) {
        if index != Self::ZERO {
            self.0[index as usize] = value;
        }
    }

    const NAMES: [&str; COUNT] = [
        "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
        "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp",
        "fp", "ra",
    ];

    #[inline(always)]
    pub const fn name(index: u32) -> &'static str {
        Self::NAMES[index as usize]
    }
}
