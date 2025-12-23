use crate::{Byte, Word};

const VRAM_SIZE: usize = 0x2000;

pub struct VRAM([u8; VRAM_SIZE]);

impl VRAM {
    pub const START: u16 = 0x8000;
    pub const END: u16 = 0x9FFF;

    pub const fn new() -> Self {
        Self([0; VRAM_SIZE])
    }

    pub const fn read(&self, address: Word) -> Byte {
        Byte::new(self.0[address.get() as usize])
    }

    pub const fn write(&mut self, address: Word, byte: Byte) {
        self.0[address.get() as usize] = byte.get();
    }
}
