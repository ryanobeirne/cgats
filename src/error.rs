use std::fmt;
use std::error::Error as StdError;
use std::convert;
use std::io;
use std::result;

#[macro_use]
#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(CgatsError::Other(format!($($tt)*))) }
}

// Custom error types for CGATS
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    CannotCompare,
    EmptyFile,
    FileError,
    FormatDataMismatch,
    IncompleteData,
    InvalidCommand,
    InvalidID,
    NoData,
    NoDataFormat,
    UnknownVendor,
    UnknownFormatType,
    WriteError,
    Other(String)
}

// Custom Result type for CgatsError
pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CgatsError::{:?}: {}", &self, &self.description())
    }
}

// Description of the Error type
impl std::error::Error for Error {
    fn description(&self) -> &str {
        use Error::*;
        match &self {
            CannotCompare      => "Cannot compare data sets!",
            EmptyFile          => "File is empty!",
            FileError          => "Problem reading file!",
            FormatDataMismatch => "DATA length does not match DATA_FORMAT length!",
            IncompleteData     => "Not enough data for the calculation!",
            InvalidCommand     => "Invalid Compare command!",
            InvalidID          => "SAMPLE_ID is not an integer!",
            NoData             => "Color Data not found!",
            NoDataFormat       => "Cannot find BEGIN_DATA_FORMAT tag!",
            UnknownVendor      => "Cannot determine Vendor!",
            UnknownFormatType  => "Unknown Data Format Type!",
            WriteError         => "Problem writing to file!",
            Other(message)     => &message,
        }
    }
}

impl convert::From<io::Error> for Error {
   fn from(e: io::Error) -> Self {
       eprintln!("{}: {:?}",e, e.kind());
       Error::FileError
   } 
} 

impl convert::From<Error> for fmt::Error {
    fn from(cge: Error) -> Self {
        eprintln!("{}", cge);
        fmt::Error
    }
}