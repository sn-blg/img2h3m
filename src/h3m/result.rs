use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::io;
use std::num::TryFromIntError;
pub type H3mResult<T> = Result<T, H3mError>;

#[derive(Debug)]
pub enum H3mError {
    Parsing(ParsingError),
    Parameter(ParameterError),
    Internal(InternalError),
    IoError(io::Error),
}

impl fmt::Display for H3mError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            H3mError::Parsing(e) => write!(
                fmt,
                "H3m file parsing error. {} Position: {}.",
                e.msg, e.position
            ),

            H3mError::Parameter(e) => write!(fmt, "Invalid parameter error. {}", e.msg),

            H3mError::Internal(e) => write!(fmt, "Internal error. {}", e.msg),

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

#[derive(Debug)]
pub struct InternalError {
    msg: String,
}

impl InternalError {
    pub fn new(msg: impl Into<String>) -> InternalError {
        InternalError { msg: msg.into() }
    }
}

impl From<io::Error> for H3mError {
    fn from(err: io::Error) -> H3mError {
        H3mError::IoError(err)
    }
}

impl From<TryFromIntError> for H3mError {
    fn from(err: TryFromIntError) -> H3mError {
        H3mError::Internal(InternalError::new(format!("Conversion error: {}.", err)))
    }
}

impl From<Infallible> for H3mError {
    fn from(err: Infallible) -> H3mError {
        H3mError::Internal(InternalError::new(format!("Conversion error: {}.", err)))
    }
}
