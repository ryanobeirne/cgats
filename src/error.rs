use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CgatsError {
    NoData,
    NoDataFormat,
    FormatDataMismatch,
    UnknownDataFormat,
    FileError,
}

pub type CgatsResult<T> = Result<T, CgatsError>;

impl fmt::Display for CgatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.description())
    }
}

impl Error for CgatsError {
    fn description(&self) -> &str {
        use CgatsError::*;
        match &self {
            NoData             => "Cannot find BEGIN_DATA tag!",
            NoDataFormat       => "Cannot find BEGIN_DATA_FORMAT tag!",
            FormatDataMismatch => "DATA_FORMAT length does not match DATA length!",
            UnknownDataFormat  => "Unknown Data Format Type!",
            FileError          => "Problem reading file!",
        }
    }
}
