use crate::Bios;

pub struct Bus {
    bios: Bios,
}

impl Bus {
    pub fn new(bios: Bios) -> Self {
        Self { bios }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            Bios::START..=Bios::END => self.bios.read(address),
            _ => panic!("Unimplemented read by 0x{address:04X}"),
        }
    }
}
