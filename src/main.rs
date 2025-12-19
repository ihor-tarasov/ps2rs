use ps2rs::{Bus, EmotionEngine};

fn main() {
    // Load bios from file
    let bios = std::fs::read("bios.bin")
        .map_err(|error| format!("Failed to read \"bios.bin\", error: {error}"))
        .unwrap();

    // Setup bus
    let mut bus = Bus { bios: &bios };

    // Setup cpu
    let mut cpu = EmotionEngine::default();

    // Run cpu
    loop {
        cpu.step(&mut bus).unwrap();
    }
}
