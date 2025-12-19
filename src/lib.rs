pub mod instruction;

mod bus;
mod cop0;
mod cpu;
mod error;
mod utils;

pub use bus::Bus;
pub use cpu::EmotionEngine;
pub use error::{Error, Result};
