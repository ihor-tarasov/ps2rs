const REGISTERS_COUNT: usize = 32;
const SELECTORS_COUNT: usize = 8;
const WORDS_COUNT: usize = REGISTERS_COUNT * SELECTORS_COUNT;

pub struct Cop0 {
    words: [u32; WORDS_COUNT],
}

impl Cop0 {
    pub const COP0_INDEX: u32 = 0;
    pub const COP0_RANDOM: u32 = 1;
    pub const COP0_ENTRYLO0: u32 = 2;
    pub const COP0_ENTRYLO1: u32 = 3;
    pub const COP0_CONTEXT: u32 = 4;
    pub const COP0_PAGEMASK: u32 = 5;
    pub const COP0_WIRED: u32 = 6;
    pub const COP0_RESERVED7: u32 = 7;

    pub const COP0_BADVADDR: u32 = 8;
    pub const COP0_COUNT: u32 = 9;
    pub const COP0_ENTRYHI: u32 = 10;
    pub const COP0_COMPARE: u32 = 11;

    pub const COP0_STATUS: u32 = 12;
    pub const COP0_CAUSE: u32 = 13;
    pub const COP0_EPC: u32 = 14;
    pub const COP0_PRID: u32 = 15;

    pub const COP0_CONFIG: u32 = 16;
    pub const COP0_LLADDR: u32 = 17;
    pub const COP0_WATCHLO: u32 = 18;
    pub const COP0_WATCHHI: u32 = 19;

    pub const COP0_XCONTEXT: u32 = 20;
    pub const COP0_RESERVED21: u32 = 21;
    pub const COP0_RESERVED22: u32 = 22;
    pub const COP0_RESERVED23: u32 = 23;

    pub const COP0_RESERVED24: u32 = 24;
    pub const COP0_RESERVED25: u32 = 25;
    pub const COP0_ECC: u32 = 26;
    pub const COP0_CACHEERR: u32 = 27;

    pub const COP0_TAGLO: u32 = 28;
    pub const COP0_TAGHI: u32 = 29;
    pub const COP0_ERROREPC: u32 = 30;
    pub const COP0_RESERVED31: u32 = 31;

    pub const fn new() -> Self {
        Self {
            words: [0; WORDS_COUNT],
        }
    }

    const fn word_index(index: u32, sel: u32) -> u32 {
        index << 3 + sel
    }

    pub const fn read(&self, index: u32, sel: u32) -> u32 {
        self.words[Self::word_index(index, sel) as usize]
    }

    pub const fn write(&mut self, index: u32, sel: u32, value: u32) {
        self.words[Self::word_index(index, sel) as usize] = value;
    }

    const NAMES: [&str; REGISTERS_COUNT] = [
        "Index",
        "Random",
        "EntryLo0",
        "EntryLo1",
        "Context",
        "PageMask",
        "Wired",
        "Reserved7",
        "BadVAddr",
        "Count",
        "EntryHi",
        "Compare",
        "Status",
        "Cause",
        "EPC",
        "PRId",
        "Config",
        "LLAddr",
        "WatchLo",
        "WatchHi",
        "XContext",
        "Reserved21",
        "Reserved22",
        "Reserved23",
        "Reserved24",
        "Reserved25",
        "ECC",
        "CacheErr",
        "TagLo",
        "TagHi",
        "ErrorEPC",
        "Reserved31",
    ];

    #[inline(always)]
    pub const fn name(index: u32) -> &'static str {
        Self::NAMES[index as usize]
    }
}
