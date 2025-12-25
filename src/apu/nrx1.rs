use crate::Byte;

#[derive(Clone, Copy)]
pub struct NRX1(u8);

#[derive(Clone, Copy)]
pub struct WaveDuty(u8);

impl WaveDuty {
    pub const PERCENT_12_5: Self = Self(0b00);
    pub const PERCENT_25: Self = Self(0b01);
    pub const PERCENT_50: Self = Self(0b10);
    pub const PERCENT_75: Self = Self(0b11);
}

impl NRX1 {
    pub const ADDRESS: u16 = 0xFF11;

    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn with_wave_duty(mut self, value: WaveDuty) -> Self {
        self.0 = (self.0 & 0b0011_1111) | (value.0 << 6);
        self
    }

    pub const fn with_initial_length_timer(mut self, value: u8) -> Self {
        self.0 = (self.0 & 0b1100_0000) | value;
        self
    }

    pub const fn wave_duty(self) -> WaveDuty {
        WaveDuty(self.0 >> 6)
    }

    pub const fn initial_length_timer(self) -> u8 {
        self.0 & 0b0011_1111
    }

    pub const fn write(self, byte: Byte) -> Self {
        Self(byte.get())
    }
}
