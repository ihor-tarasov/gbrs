use std::fmt;

use crate::{Byte, Word};

#[derive(Debug)]
pub enum Error {
    Opcode(Byte),
    CBOpcode(Byte),
    Read(Word),
    Write(Word),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Opcode(byte) => write!(f, "Unknown opcode {byte}"),
            Error::CBOpcode(byte) => write!(f, "Unknown CB opcode {byte}"),
            Error::Read(word) => write!(f, "Failed to read by {word}"),
            Error::Write(word) => write!(f, "Failed to write by {word}"),
        }
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;
