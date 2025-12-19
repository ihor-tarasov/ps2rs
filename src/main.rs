use std::fs::File;

use ps2rs::{Bus, EmotionEngine};

fn main() {
    // Setup bus
    let mut bios = File::open("bios.bin")
        .map_err(|error| format!("Failed to open bios.bin file, error: {error}"))
        .unwrap();
    let mut bus = Bus::new(&mut bios)
        .map_err(|error| format!("Failed to read bios.bin file, error: {error}"))
        .unwrap();

    // Setup cpu
    let mut cpu = EmotionEngine::default();

    // Run cpu
    loop {
        cpu.step(&mut bus).unwrap();
    }
}
