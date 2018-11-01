use std::error::Error;
use std::fmt;
use std::convert;
use std::io;

// Custom error types for CGATS
#[derive(Debug)]
pub enum CgatsError {
    NoData,
    NoDataFormat,
    FormatDataMismatch,
    UnknownFormatType,
    FileError,
    EmptyFile,
}

// Custom Result type for CgatsError
pub type CgatsResult<T> = Result<T, CgatsError>;

impl fmt::Display for CgatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CgatsError::{:?}: {}", &self, &self.description())
    }
}

// Description of the Error type
impl Error for CgatsError {
    fn description(&self) -> &str {
        use CgatsError::*;
        match &self {
            NoData             => "Cannot find BEGIN_DATA tag!",
            NoDataFormat       => "Cannot find BEGIN_DATA_FORMAT tag!",
            FormatDataMismatch => "DATA length does not match DATA_FORMAT length!",
            UnknownFormatType  => "Unknown Data Format Type!",
            FileError          => "Problem reading file!",
            EmptyFile          => "File is empty!",
        }
    }
}

impl convert::From<io::Error> for CgatsError {
   fn from(e: io::Error) -> Self {
       eprintln!("{}",e);
       CgatsError::FileError
   } 
} 
