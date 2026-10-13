use std::{error, fmt::Display, io};

use serde::ser;

#[derive(Debug)]
pub enum ErrorCode {
    Custom(String),
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Parse(ErrorCode, usize),
    Io(std::io::Error),
    NotImplemented,
    BufferFull,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(error_code, pos) => write!(f, "{} at byte {}", error_code, pos),
            Error::Io(error) => error.fmt(f),
            Error::NotImplemented => write!(f, "Not Implemented"),
        }
    }
}
impl error::Error for Error {}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Parse(ErrorCode::Custom(msg.to_string()), 0)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
