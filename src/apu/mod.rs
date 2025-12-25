mod channel;
mod nr52;

pub use channel::Channel;
pub use nr52::NR52;

pub struct APU {
    pub nr52: NR52,
}

impl APU {
    pub const fn new() -> Self {
        Self { nr52: NR52::new() }
    }

    pub const fn reset(&mut self) {
        // TODO: Clear all registers
    }
}
