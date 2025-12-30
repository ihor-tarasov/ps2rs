use crate::Bios;

pub trait BusUnit {
    fn read_u8(&self, address: u32) -> u8;

    fn read_u32(&self, address: u32) -> u32 {
        let b0 = self.read_u8(address);
        let b1 = self.read_u8(address + 1);
        let b2 = self.read_u8(address + 2);
        let b3 = self.read_u8(address + 3);
        b0 as u32 | (b1 as u32) << 8 | (b2 as u32) << 16 | (b3 as u32) << 24
    }
}

#[derive(Debug)]
pub enum BusError {
    TlbNotImplemented(u32),
    UnknownAddress(u32),
}

pub type BusResult<T = ()> = Result<T, BusError>;

pub struct Bus {
    pub bios: Bios,
}

impl Bus {
    const KUSEG_START: u32 = 0x0000_0000;
    const KUSEG_END: u32 = 0x7FFF_FFFF;
    const KSEG0_START: u32 = 0x8000_0000;
    const KSEG0_END: u32 = 0x9FFF_FFFF;
    const KSEG1_START: u32 = 0xA000_0000;
    const KSEG1_END: u32 = 0xBFFF_FFFF;
    const TLB_START: u32 = 0xC000_0000;
    const TLB_END: u32 = 0xFFFF_FFFF;

    fn read_u32_phys(&self, address: u32) -> BusResult<u32> {
        match address {
            Bios::START..=Bios::END => Ok(self.bios.read_u32(address - Bios::START)),
            _ => Err(BusError::UnknownAddress(address)),
        }
    }

    const fn virt_to_phys(address: u32) -> BusResult<u32> {
        match address {
            Self::KUSEG_START..=Self::KUSEG_END => Ok(address),
            Self::KSEG0_START..=Self::KSEG0_END => Ok(address - Self::KSEG0_START),
            Self::KSEG1_START..=Self::KSEG1_END => Ok(address - Self::KSEG1_START),
            Self::TLB_START..=Self::TLB_END => Err(BusError::TlbNotImplemented(address)),
        }
    }

    pub fn read_u32(&self, address: u32) -> BusResult<u32> {
        self.read_u32_phys(Self::virt_to_phys(address)?)
    }
}
