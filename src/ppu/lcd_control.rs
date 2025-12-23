#[repr(u8)]
#[derive(Clone, Copy)]
pub enum LCDCFlag {
    BGWindowEnable = 0b0000_0001,
    OBJEnable = 0b0000_0010,
    OBJSize = 0b0000_0100,
    BGTileMap = 0b0000_1000,
    BGWindowTiles = 0b0001_0000,
    WindowEnable = 0b0010_0000,
    WindowTileMap = 0b0100_0000,
    LCDPPUEnable = 0b1000_0000,
}

#[derive(Clone, Copy)]
pub struct LCDControl(u8);

impl LCDControl {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn get(self, flag: LCDCFlag) -> bool {
        self.0 & (flag as u8) != 0
    }

    pub const fn with(mut self, flag: LCDCFlag, set: bool) -> Self {
        if set {
            self.0 |= flag as u8;
        } else {
            self.0 &= !(flag as u8);
        }
        self
    }
}
