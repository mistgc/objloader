use std::{string, num};

#[derive(Debug)]
pub enum Error {
    IndexOutOfBound,
    ParseingPositionError,

    FromUtf8Error,

    ParseIntError,
    ParseFloatError,
}

impl From<string::FromUtf8Error> for Error {
    fn from(_: string::FromUtf8Error) -> Self {
        Self::FromUtf8Error
    }
}

impl From<num::ParseIntError> for Error {
    fn from(_: num::ParseIntError) -> Self {
        Self::ParseIntError
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(_: num::ParseFloatError) -> Self {
        Self::ParseFloatError
    }
}
