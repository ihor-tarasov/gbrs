use crate::Byte;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum LCDSFlag {
    LYCEqLY = 2,
    Mode0Int = 3,
    Mode1Int = 4,
    Mode2Int = 5,
    LYCInt = 6,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PPUMode(u8);

impl PPUMode {
    pub const HORIZONTAL_BLANK: Self = Self(0);
    pub const VERTICAL_BLANK: Self = Self(1);
    pub const OAM_SCAN: Self = Self(2);
    pub const DRAWING_PIXELS: Self = Self(3);
}

#[derive(Clone, Copy)]
pub struct LCDStatus(Byte);

impl LCDStatus {
    pub const fn new() -> Self {
        Self(Byte::ZERO)
    }

    pub const fn ppu_mode(self) -> PPUMode {
        PPUMode(self.0.get() & 0b0000_0011)
    }

    pub const fn get(self, flag: LCDSFlag) -> bool {
        self.0.bit(flag as u8)
    }

    pub const fn with(self, flag: LCDSFlag, set: bool) -> Self {
        Self(self.0.with_bit(flag as u8, set))
    }
}
