use std::fmt;

use crate::{Byte, Word};

#[derive(Debug)]
pub enum Error {
    Opcode(Byte),
    Address(Word),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Opcode(byte) => write!(f, "Unknown opcode {byte}"),
            Error::Address(word) => write!(f, "Failed to read by {word}"),
        }
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;
