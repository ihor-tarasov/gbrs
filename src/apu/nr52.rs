use crate::{Byte, apu::Channel};

const AUDIO_ON_OFF_BIT: u8 = 7;
const WRITE_DATA_MASK: u8 = 0b1000_0000;

#[derive(Clone, Copy)]
pub struct NR52(Byte);

impl NR52 {
    pub const ADDRESS: u16 = 0xFF26;

    pub const fn new() -> Self {
        Self(Byte::ZERO)
    }

    pub const fn with_audio_on(self, on: bool) -> Self {
        Self(self.0.with_bit(AUDIO_ON_OFF_BIT, on))
    }

    pub const fn with_channel_on(self, channel: Channel, on: bool) -> Self {
        Self(self.0.with_bit(channel as u8, on))
    }

    pub const fn audio_on(self) -> bool {
        self.0.bit(AUDIO_ON_OFF_BIT)
    }

    pub const fn channel_on(self, channel: Channel) -> bool {
        self.0.bit(channel as u8)
    }

    pub const fn write(self, byte: Byte) -> Self {
        Self(Byte::new(byte.get() & WRITE_DATA_MASK))
    }
}
