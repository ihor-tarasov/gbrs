use crate::Word;

#[derive(Default)]
pub struct Cpu {
    pub pc: Word,
    pub sp: Word,
}
