use serde::de;
use serde::ser;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    Custom(String),
    IO(::std::io::Error),
    Utf8(::std::str::Utf8Error),
    Parse(ParseError),
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

impl From<::std::str::Utf8Error> for Error {
    fn from(err: ::std::str::Utf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::Parse(err)
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Self {
        Error::IO(err)
    }
}

#[derive(Debug)]
pub enum ParseError {
    NoKey,
    NoValue,
    InvalidValue,
}
