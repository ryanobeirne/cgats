use super::*;

#[derive(Debug)]
pub enum CgatsError {
    NoDataFound,
    NoDataFormatFound,
    FormatDataMismatch,
    UnknownDataFormat,
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
            NoDataFound        => "Cannot find BEGIN_DATA tag!",
            NoDataFormatFound  => "Cannot find BEGIN_DATA_FORMAT tag!",
            FormatDataMismatch => "DATA_FORMAT length does not match DATA length!",
            UnknownDataFormat  => "Unknown Data Format Type!"
        }
    }
}
