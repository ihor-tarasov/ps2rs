use std::ops::RangeInclusive;

use crate::{Error, Result, utils};

pub struct Bus<'a> {
    pub bios: &'a [u8],
}

impl<'a> Bus<'a> {
    pub const RANGE_BIOS: RangeInclusive<u32> = 0x1FC0_0000..=0x2000_0000;

    pub const BIOS_START: u32 = 0x1FC0_0000;
    pub const BIOS_END: u32 = 0x1FFF_FFFF;

    pub fn new(bios: &'a [u8]) -> Self {
        Self { bios }
    }

    pub const fn read4(&self, address: u32) -> Result<[u8; 4]> {
        match address {
            Self::BIOS_START..=Self::BIOS_END => {
                Ok(utils::read4_from_slice(self.bios, address & 0x3F_FFFF))
            }
            _ => Err(Error::ReadU32(address)),
        }
    }
}
