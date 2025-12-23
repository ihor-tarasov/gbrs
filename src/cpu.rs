use crate::Bus;

pub struct Cpu {
    pc: u16,
}

impl Cpu {
    const INITIAL_PC: u16 = 0x0000;

    pub const fn new() -> Self {
        Self {
            pc: Self::INITIAL_PC,
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        match bus.read(self.pc) {
            opcode => panic!("Unimplemented opcode: 0x{opcode:02X} at 0x{:04X}", self.pc),
        }
    }
}
