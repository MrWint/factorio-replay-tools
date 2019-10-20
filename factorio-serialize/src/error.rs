use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

/// The result of a serialization or deserialization operation.
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
  kind: ErrorKind,
  position: u64,
}
impl Error {
  pub fn from_io(error: std::io::Error, position: u64) -> Self {
    Error { kind: ErrorKind::Io(error), position, }
  }
  pub fn from_utf8(error: FromUtf8Error, position: u64) -> Self {
    Error { kind: ErrorKind::InvalidUtf8Encoding(error), position, }
  }
  pub fn custom(error: String, position: u64) -> Self {
    Error { kind: ErrorKind::Custom(error), position, }
  }
}

#[derive(Debug)]
pub enum ErrorKind {
  Io(std::io::Error),
  /// Returned if the deserializer attempts to deserialize a string that is not valid utf8
  InvalidUtf8Encoding(FromUtf8Error),
  /// A custom error message
  Custom(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind {
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::InvalidUtf8Encoding(ref err) => Some(err),
            ErrorKind::Custom(_) => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self.kind {
            ErrorKind::Io(ref err) => write!(fmt, "At position {:x}: IO error: {}", self.position, err),
            ErrorKind::InvalidUtf8Encoding(ref err) => write!(fmt, "At position {:x}: UTF-8 error: {}", self.position, err),
            ErrorKind::Custom(ref s) => write!(fmt, "At position {:x}: error: {}", self.position, s),
        }
    }
}
