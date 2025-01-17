use std::fmt::{Debug, Formatter, Result};

pub enum Error {
    AlreadyTrimmed,
    InvalidPath,
    Io,
    NotSupported,
    TooSmall,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let message = match self {
            Error::AlreadyTrimmed => "Already trimmed",
            Error::InvalidPath => "Invalid path",
            Error::Io => "I/O Error",
            Error::NotSupported => "File is not supported",
            Error::TooSmall => "File is too small",
        };
        write!(f, "{message}")
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Platform {
    Gba,
    Nds,
}

impl Platform {
    pub(crate) fn logoaddress(self) -> u64 {
        match self {
            Platform::Nds => 0x004,
            Platform::Gba => 0x0C0,
        }
    }
}
