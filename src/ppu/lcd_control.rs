use crate::Byte;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum LCDCFlag {
    BGWindowEnable,
    OBJEnable,
    OBJSize,
    BGTileMap,
    BGWindowTiles,
    WindowEnable,
    WindowTileMap,
    LCDPPUEnable,
}

#[derive(Clone, Copy)]
pub struct LCDControl(Byte);

impl LCDControl {
    pub const fn new() -> Self {
        Self(Byte::ZERO)
    }

    pub const fn get(self, flag: LCDCFlag) -> bool {
        self.0.bit(flag as u8)
    }

    pub const fn with(self, flag: LCDCFlag, set: bool) -> Self {
        Self(self.0.with_bit(flag as u8, set))
    }
}
