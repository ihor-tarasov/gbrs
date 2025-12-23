use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::Byte;

#[derive(Clone, Copy, Default)]
pub struct Word(u16);

impl Word {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u16 {
        self.0
    }

    pub const fn from_bytes(hi: Byte, lo: Byte) -> Self {
        Self(u16::from_le_bytes([lo.get(), hi.get()]))
    }
}

impl Add for Word {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_add(rhs.0))
    }
}

impl Add<u16> for Word {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0.wrapping_add(rhs))
    }
}

impl Add<Word> for u16 {
    type Output = Word;

    fn add(self, rhs: Word) -> Self::Output {
        Word(self.wrapping_add(rhs.0))
    }
}

impl AddAssign for Word {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_add(rhs.0);
    }
}

impl AddAssign<u16> for Word {
    fn add_assign(&mut self, rhs: u16) {
        self.0 = self.0.wrapping_add(rhs)
    }
}

impl Sub for Word {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_sub(rhs.0))
    }
}

impl Sub<u16> for Word {
    type Output = Self;

    fn sub(self, rhs: u16) -> Self::Output {
        Self(self.0.wrapping_sub(rhs))
    }
}

impl Sub<Word> for u16 {
    type Output = Word;

    fn sub(self, rhs: Word) -> Self::Output {
        Word(self.wrapping_sub(rhs.0))
    }
}

impl SubAssign for Word {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_sub(rhs.0);
    }
}

impl SubAssign<u16> for Word {
    fn sub_assign(&mut self, rhs: u16) {
        self.0 = self.0.wrapping_sub(rhs)
    }
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}
