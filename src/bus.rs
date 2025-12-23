use crate::{Bios, Byte, Error, Result, Word};

pub struct Bus {
    bios: Bios,
}

impl Bus {
    pub fn new(bios: Bios) -> Self {
        Self { bios }
    }

    pub fn read_byte(&self, address: Word) -> Result<Byte> {
        match address.get() {
            Bios::START..=Bios::END => Ok(self.bios.read_byte(address)),
            _ => Err(Error::Address(address)),
        }
    }

    pub fn read_word(&self, address: Word) -> Result<Word> {
        Ok(Word::from_bytes(
            self.read_byte(address)?,
            self.read_byte(address + 1)?,
        ))
    }
}
