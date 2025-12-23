use crate::{Byte, Flags, Word};

#[derive(Default)]
pub struct CPU {
    pub a: Byte,
    pub f: Flags,
    pub h: Byte,
    pub l: Byte,
    pub pc: Word,
    pub sp: Word,
}

impl CPU {
    pub const fn hl(&self) -> Word {
        Word::from_bytes(self.h, self.l)
    }

    pub fn dec_hl(&mut self) {
        if self.l == Byte::ZERO {
            self.l = Byte::MAX;
            self.h -= 1;
        } else {
            self.l -= 1;
        }
    }
}
