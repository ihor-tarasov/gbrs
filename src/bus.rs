use crate::{Bios, Byte, Error, Result, Word};

pub struct Bus {
    bios: Bios,
}

impl Bus {
    pub fn new(bios: Bios) -> Self {
        Self { bios }
    }

    pub fn read(&self, address: Word) -> Result<Byte> {
        match address.get() {
            Bios::START..=Bios::END => Ok(self.bios.read(address)),
            _ => Err(Error::Read(address)),
        }
    }

    pub fn write(&mut self, address: Word, byte: Byte) -> Result {
        match address.get() {
            Bios::START..=Bios::END => {
                // Ignore write to BIOS
                log::warn!("Trying to write {byte} by {address} when executing BIOS");
                Ok(())
            }
            _ => Err(Error::Write(address)),
        }
    }
}
