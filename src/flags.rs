use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Flag {
    Zero = 0b1000_0000,
    Subtract = 0b0100_0000,
    Half = 0b0010_0000,
    Carry = 0b0001_0000,
}

#[derive(Default, Clone, Copy)]
pub struct Flags(u8);

impl Flags {
    pub const fn new(z: bool, n: bool, h: bool, c: bool) -> Self {
        Self(0)
            .with(Flag::Zero, z)
            .with(Flag::Subtract, n)
            .with(Flag::Half, h)
            .with(Flag::Carry, c)
    }

    pub const fn with(mut self, flag: Flag, set: bool) -> Self {
        if set {
            self.0 |= flag as u8;
        } else {
            self.0 &= !(flag as u8);
        }
        self
    }

    pub const fn get(self, flag: Flag) -> bool {
        self.0 & (flag as u8) != 0
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
