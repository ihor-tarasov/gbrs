use std::fmt;

use crate::Byte;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Flag {
    Zero = 7,
    Subtract = 6,
    Half = 5,
    Carry = 4,
}

#[derive(Default, Clone, Copy)]
pub struct Flags(Byte);

impl Flags {
    pub const fn new(z: bool, n: bool, h: bool, c: bool) -> Self {
        Self(Byte::ZERO)
            .with(Flag::Zero, z)
            .with(Flag::Subtract, n)
            .with(Flag::Half, h)
            .with(Flag::Carry, c)
    }

    pub const fn with(self, flag: Flag, set: bool) -> Self {
        Self(self.0.with_bit(flag as u8, set))
    }

    pub const fn get(self, flag: Flag) -> bool {
        self.0.bit(flag as u8)
    }
}

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Z:{} N: {} H: {} C: {}]",
            self.get(Flag::Zero) as u8,
            self.get(Flag::Subtract) as u8,
            self.get(Flag::Half) as u8,
            self.get(Flag::Carry) as u8
        )
    }
}
