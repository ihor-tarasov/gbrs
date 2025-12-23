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
            _ => Err(Error::Address(address)),
        }
    }
}
