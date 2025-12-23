mod lcd_control;
mod lcd_status;
mod vram;

pub use lcd_control::{LCDCFlag, LCDControl};
pub use lcd_status::{LCDSFlag, LCDStatus, PPUMode};
pub use vram::VRAM;

pub struct PPU {
    pub lcdc: LCDControl,
    pub stat: LCDStatus,
    pub vram: VRAM,
}

impl PPU {
    pub const fn new() -> Self {
        Self {
            lcdc: LCDControl::new(),
            stat: LCDStatus::new(),
            vram: VRAM::new(),
        }
    }

    pub fn vram_blocked_for_cpu(&self) -> bool {
        self.lcdc.get(LCDCFlag::LCDPPUEnable) && self.stat.ppu_mode() == PPUMode::DRAWING_PIXELS
    }
}
