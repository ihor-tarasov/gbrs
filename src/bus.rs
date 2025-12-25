use crate::{
    BIOS, Byte, Error, Result, Word,
    apu::{APU, NR52},
    ppu::{PPU, VRAM},
};

pub struct Bus {
    pub bios: BIOS,
    pub ppu: PPU,
    pub apu: APU,
}

impl Bus {
    pub fn read(&self, address: Word) -> Result<Byte> {
        match address.get() {
            BIOS::START..=BIOS::END => Ok(self.bios.read(address)),
            VRAM::START..=VRAM::END => Ok(if self.ppu.vram_blocked_for_cpu() {
                Byte::MAX
            } else {
                self.ppu.vram.read(address - VRAM::START)
            }),
            _ => Err(Error::Read(address)),
        }
    }

    pub fn write(&mut self, address: Word, byte: Byte) -> Result {
        match address.get() {
            BIOS::START..=BIOS::END => {
                // Ignore write to BIOS
                log::warn!("Trying to write {byte} by {address} when executing BIOS");
                Ok(())
            }
            VRAM::START..=VRAM::END => Ok(if !self.ppu.vram_blocked_for_cpu() {
                self.ppu.vram.write(address - VRAM::START, byte);
            }),
            NR52::ADDRESS => {
                let nr52 = self.apu.nr52.write(byte);
                if !nr52.audio_on() {
                    self.apu.reset();
                }
                self.apu.nr52 = nr52;
                Ok(())
            }
            _ => Err(Error::Write(address)),
        }
    }
}
