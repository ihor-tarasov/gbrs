use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Byte(u8);

impl Byte {
    pub const ZERO: Self = Self(0x00);
    pub const MAX: Self = Self(0xFF);

    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn bit(self, bit: u8) -> bool {
        self.0 & (1 << bit) != 0
    }

    pub const fn with_bit(mut self, bit: u8, set: bool) -> Self {
        if set {
            self.0 |= 1 << bit;
        } else {
            self.0 &= !(1 << bit);
        }
        self
    }
}

impl Add for Byte {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_add(rhs.0))
    }
}

impl Add<u8> for Byte {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self(self.0.wrapping_add(rhs))
    }
}

impl Add<Byte> for u8 {
    type Output = Byte;

    fn add(self, rhs: Byte) -> Self::Output {
        Byte(self.wrapping_add(rhs.0))
    }
}

impl AddAssign for Byte {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_add(rhs.0);
    }
}

impl AddAssign<u8> for Byte {
    fn add_assign(&mut self, rhs: u8) {
        self.0 = self.0.wrapping_add(rhs)
    }
}

impl Sub for Byte {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_sub(rhs.0))
    }
}

impl Sub<u8> for Byte {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        Self(self.0.wrapping_sub(rhs))
    }
}

impl Sub<Byte> for u8 {
    type Output = Byte;

    fn sub(self, rhs: Byte) -> Self::Output {
        Byte(self.wrapping_sub(rhs.0))
    }
}

impl SubAssign for Byte {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_sub(rhs.0);
    }
}

impl SubAssign<u8> for Byte {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 = self.0.wrapping_sub(rhs)
    }
}

impl fmt::Debug for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}
