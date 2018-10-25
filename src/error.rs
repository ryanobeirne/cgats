use std::error::Error;
use std::fmt;
use std::convert;
use std::io;

#[derive(Debug)]
pub enum CgatsError {
    NoData,
    NoDataFormat,
    FormatDataMismatch,
    UnknownFormatType,
    FileError,
}

pub type CgatsResult<T> = Result<T, CgatsError>;

impl fmt::Display for CgatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", &self, &self.description())
    }
}

impl Error for CgatsError {
    fn description(&self) -> &str {
        use CgatsError::*;
        match &self {
            NoData             => "Cannot find BEGIN_DATA tag!",
            NoDataFormat       => "Cannot find BEGIN_DATA_FORMAT tag!",
            FormatDataMismatch => "DATA_FORMAT length does not match DATA length!",
            UnknownFormatType  => "Unknown Data Format Type!",
            FileError          => "Problem reading file!",
        }
    }
}

impl convert::From<io::Error> for CgatsError {
   fn from(e: io::Error) -> Self {
       eprintln!("{}",e);
       CgatsError::FileError
   } 
} 
