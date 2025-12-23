use std::{fs::File, io, path::Path};

use crate::{Byte, Word};

const BIOS_SIZE: usize = 0x100;

pub struct Bios([u8; BIOS_SIZE]);

impl Bios {
    pub const START: u16 = 0x0000;
    pub const END: u16 = 0x00FF;

    pub fn from_read<R>(read: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let mut data = [0; BIOS_SIZE];
        read.read_exact(&mut data)?;
        Ok(Self(data))
    }

    pub fn load<P>(path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path)?;
        Self::from_read(&mut file)
    }

    pub fn read(&self, address: Word) -> Byte {
        Byte::new(self.0[address.get() as usize])
    }
}
