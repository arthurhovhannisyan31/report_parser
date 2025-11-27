use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParsingError {
  IO(io::Error),
  ParseInt(ParseIntError),
  ParseTxType(TxTypeError),
  ParseBin {
    source: io::Error,
    description: String,
  },
}

#[derive(Debug)]
pub enum SerializeError {
  NotFound,
}

#[derive(Debug)]
pub enum TxTypeError {
  InvalidSting(String),
  InvalidNumber(u8),
}

impl Display for ParsingError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::IO(err) => write!(f, "{:?}", err),
      Self::ParseInt(err) => write!(f, "{:?}", err),
      Self::ParseTxType(err) => write!(f, "{:?}", err),
      Self::ParseBin {
        source,
        description,
      } => {
        write!(f, "{:?}\n {:?}", description, source)
      }
    }
  }
}

impl std::error::Error for ParsingError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::IO(err) => Some(err),
      Self::ParseInt(err) => Some(err),
      Self::ParseTxType(err) => None,
      Self::ParseBin {
        source,
        description,
      } => Some(source),
    }
  }
}

impl From<io::Error> for ParsingError {
  fn from(err: io::Error) -> Self {
    Self::IO(err)
  }
}

impl From<ParseIntError> for ParsingError {
  fn from(err: ParseIntError) -> Self {
    Self::ParseInt(err)
  }
}

impl From<TxTypeError> for ParsingError {
  fn from(value: TxTypeError) -> Self {
    Self::ParseTxType(value)
  }
}

impl Display for TxTypeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidSting(s) => {
        write!(f, "Invalid string transaction type: {:?}", s)
      }
      Self::InvalidNumber(n) => {
        write!(f, "Invalid number transaction type: {:?}", n)
      }
    }
  }
}
