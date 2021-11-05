use std::error::Error;
use std::fmt;
use std::io;

pub type H3mResult<T> = Result<T, H3mError>;

#[derive(Debug)]
pub enum H3mError {
    Parsing(ParsingError),
    Parameter(ParameterError),
    IoError(io::Error),
}

impl fmt::Display for H3mError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            H3mError::Parsing(e) => write!(
                fmt,
                "H3m file parsing error. {} Position: {}.",
                e.msg, e.position
            ),

            H3mError::Parameter(e) => write!(fmt, "Invalid parameter error. {}", e.msg),

            H3mError::IoError(e) => fmt::Display::fmt(e, fmt),
        }
    }
}

impl Error for H3mError {}

#[derive(Debug)]
pub struct ParsingError {
    position: u64,
    msg: String,
}

impl ParsingError {
    pub fn new(position: u64, msg: impl Into<String>) -> ParsingError {
        ParsingError {
            position,
            msg: msg.into(),
        }
    }
}

#[derive(Debug)]
pub struct ParameterError {
    msg: String,
}

impl ParameterError {
    pub fn new(msg: impl Into<String>) -> ParameterError {
        ParameterError { msg: msg.into() }
    }
}

impl From<io::Error> for H3mError {
    fn from(err: io::Error) -> H3mError {
        H3mError::IoError(err)
    }
}
