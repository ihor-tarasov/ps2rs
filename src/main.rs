use ps2rs::{Bios, Bus, EmotionEngine};

fn setup_bus() -> Bus {
    Bus {
        bios: Bios::open("bios.bin").unwrap(),
    }
}

fn main() {
    let mut cpu = EmotionEngine::new();
    let mut bus = setup_bus();
    loop {
        match cpu.step(&mut bus) {
            Ok(_) => {}
            Err(error) => {
                println!("Error: {error:?}");
                break;
            }
        }
    }
}
