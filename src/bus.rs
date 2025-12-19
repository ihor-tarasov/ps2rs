use std::io;

use crate::{Error, Result, utils};

pub struct Bus {
    bios: Box<[u8]>,
    rdram: Box<[u8]>,
}

impl Bus {
    const BIOS_SIZE: usize = 0x0040_0000;
    const RDRAM_SIZE: usize = 0x0200_0000;
    const BIOS_START: u32 = 0x1FC0_0000;
    const BIOS_END: u32 = 0x1FFF_FFFF;
    const RDRAM_START: u32 = 0x0000_0000;
    const RDRAM_END: u32 = 0x01FF_FFFF;

    pub fn new<R>(read: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let mut bios = Vec::with_capacity(Self::BIOS_SIZE);
        if read.read_to_end(&mut bios)? != Self::BIOS_SIZE {
            return Err(io::Error::other(format!(
                "Unexpected file size, expected: {} bytes",
                Self::BIOS_SIZE
            )));
        }

        Ok(Self {
            bios: bios.into_boxed_slice(),
            rdram: vec![0; Self::RDRAM_SIZE].into_boxed_slice(),
        })
    }

    pub fn read4(&self, address: u32) -> Result<[u8; 4]> {
        match address {
            Self::RDRAM_START..=Self::RDRAM_END => {
                Ok(utils::read4_from_slice(self.rdram.as_ref(), address))
            }
            Self::BIOS_START..=Self::BIOS_END => Ok(utils::read4_from_slice(
                self.bios.as_ref(),
                address & 0x3F_FFFF,
            )),
            _ => Err(Error::ReadU32(address)),
        }
    }
}
