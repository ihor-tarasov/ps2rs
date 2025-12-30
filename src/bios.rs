use std::{fs::File, io, path::Path};

use crate::BusUnit;

pub struct Bios {
    bytes: Vec<u8>,
}

impl Bios {
    pub const START: u32 = 0x1FC0_0000;
    pub const END: u32 = 0x1FFF_FFFF;

    const SIZE: usize = (Self::END - Self::START) as usize + 1;

    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let mut bytes = Vec::with_capacity(Self::SIZE);
        let size = reader.read_to_end(&mut bytes)?;
        if size != Self::SIZE {
            return Err(io::Error::other("Wrong bios size"));
        }
        Ok(Self::new(bytes))
    }

    pub fn open<P>(path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path)?;
        Self::read(&mut file)
    }
}

impl BusUnit for Bios {
    fn read_u8(&self, address: u32) -> u8 {
        self.bytes[address as usize]
    }
}
