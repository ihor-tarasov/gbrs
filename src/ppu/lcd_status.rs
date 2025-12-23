#[repr(u8)]
#[derive(Clone, Copy)]
pub enum LCDSFlag {
    LYCEqLY = 0b0000_0100,
    Mode0Int = 0b0000_1000,
    Mode1Int = 0b0001_0000,
    Mode2Int = 0b0010_0000,
    LYCInt = 0b0100_0000,
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
pub struct LCDStatus(u8);

impl LCDStatus {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn ppu_mode(self) -> PPUMode {
        PPUMode(self.0 & 0b0000_0011)
    }

    pub const fn get(self, flag: LCDSFlag) -> bool {
        self.0 & (flag as u8) != 0
    }

    pub const fn with(mut self, flag: LCDSFlag, set: bool) -> Self {
        if set {
            self.0 |= flag as u8;
        } else {
            self.0 &= !(flag as u8);
        }
        self
    }
}
