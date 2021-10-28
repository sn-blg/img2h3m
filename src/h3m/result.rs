use std::error::Error;
use std::fmt;
use std::io;

pub type H3mResult<T> = Result<T, H3mError>;

#[derive(Debug)]
pub enum H3mError {
    ParseError,
    IoError(io::Error),
}

impl fmt::Display for H3mError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            H3mError::ParseError => write!(fmt, "parse error"),
            H3mError::IoError(e) => fmt::Display::fmt(e, fmt),
        }
    }
}

impl Error for H3mError {}

#[derive(Debug)]
pub struct DecodingError {
    underlying: Option<Box<dyn Error + Send + Sync>>,
}

#[derive(Debug)]
pub struct EncodingError {
    underlying: Option<Box<dyn Error + Send + Sync>>,
}

impl From<io::Error> for H3mError {
    fn from(err: io::Error) -> H3mError {
        H3mError::IoError(err)
    }
}
