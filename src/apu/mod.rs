mod channel;
mod nr52;
mod nrx1;

pub use channel::Channel;
pub use nr52::NR52;
pub use nrx1::{NRX1, WaveDuty};

pub struct APU {
    pub nr11: NRX1,
    pub nr52: NR52,
}

impl APU {
    pub const fn new() -> Self {
        Self {
            nr11: NRX1::new(),
            nr52: NR52::new(),
        }
    }

    pub const fn reset(&mut self) {
        // TODO: Clear all registers
        self.nr11 = NRX1::new();
    }
}
