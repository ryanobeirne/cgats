/// # CGATS error-handling module
use std::fmt;
use std::error::Error as StdError;
use std::result;

/// The CGATS error type
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    CannotCompare,
    EmptyFile,
    ReadError,
    FormatDataMismatch,
    IncompleteData,
    //InvalidCommand,
    InvalidID,
    NoData,
    NoDataFormat,
    UnknownVendor,
    UnknownField,
    WriteError,
    Other(String)
}

// Custom Result type for CgatsError
//pub type CgatsResult<T> = result::Result<T, Error>;
pub type Result<T> = result::Result<T, BoxError>;
pub type BoxError = Box<dyn std::error::Error>;

#[macro_export]
macro_rules! boxerr {
    ($x: expr) => {
        Err(Box::new($x))
    }
}

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
            ReadError          => "Problem reading file!",
            FormatDataMismatch => "DATA length does not match DATA_FORMAT length!",
            IncompleteData     => "Not enough data for the calculation!",
            //InvalidCommand     => "Invalid Compare command!",
            InvalidID          => "SAMPLE_ID is not an integer!",
            NoData             => "DATA not found!",
            NoDataFormat       => "DATA_FORMAT tag not found!",
            UnknownVendor      => "Cannot determine Vendor!",
            UnknownField       => "Unknown Data Field Type!",
            WriteError         => "Problem writing to file!",
            Other(message)     => message.as_str(),
        }
    }
}

