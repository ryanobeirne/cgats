use std::error::Error;
use std::fmt;
use std::convert;
use std::io;

// Custom error types for CGATS
#[derive(Debug)]
pub enum CgatsError {
    CannotCompare,
    EmptyFile,
    FileError,
    FormatDataMismatch,
    InvalidCommand,
    InvalidID,
    NoData,
    NoDataFormat,
    UnknownCgatsType,
    UnknownFormatType,
    WriteError,
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
            CannotCompare      => "Cannot compare data sets!",
            EmptyFile          => "File is empty!",
            FileError          => "Problem reading file!",
            FormatDataMismatch => "DATA length does not match DATA_FORMAT length!",
            InvalidCommand     => "Invalid Compare command!",
            InvalidID          => "SAMPLE_ID is not an integer!",
            NoData             => "Cannot find BEGIN_DATA tag!",
            NoDataFormat       => "Cannot find BEGIN_DATA_FORMAT tag!",
            UnknownCgatsType   => "Cannot determine CGATS type!",
            UnknownFormatType  => "Unknown Data Format Type!",
            WriteError         => "Problem writing to file!",
        }
    }
}

impl convert::From<io::Error> for CgatsError {
   fn from(e: io::Error) -> Self {
       eprintln!("{}",e);
       CgatsError::FileError
   } 
} 

impl convert::From<CgatsError> for fmt::Error {
    fn from(cge: CgatsError) -> Self {
        eprintln!("{}", cge);
        fmt::Error
    }
}