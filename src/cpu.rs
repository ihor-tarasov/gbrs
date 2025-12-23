use crate::{Byte, Flags, Word};

#[derive(Default)]
pub struct Cpu {
    pub a: Byte,
    pub f: Flags,
    pub h: Byte,
    pub l: Byte,
    pub pc: Word,
    pub sp: Word,
}
