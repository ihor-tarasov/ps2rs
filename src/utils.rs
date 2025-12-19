pub const fn read4_from_slice(slice: &[u8], address: u32) -> [u8; 4] {
    [
        slice[(address + 0) as usize],
        slice[(address + 1) as usize],
        slice[(address + 2) as usize],
        slice[(address + 3) as usize],
    ]
}
