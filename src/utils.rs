pub const fn read_u32_from_slice(slice: &[u8], address: u32) -> u32 {
    u32::from_le_bytes([
        slice[(address + 0) as usize],
        slice[(address + 1) as usize],
        slice[(address + 2) as usize],
        slice[(address + 3) as usize],
    ])
}
