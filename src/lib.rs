use std::fmt;
use std::path::Path;
use std::fs::File;
use std::convert::TryFrom;

pub mod field;
pub mod read;
mod vendor;
#[macro_use]
mod error;

use field::*;
use vendor::*; 
use error::*;

pub use error::*;

#[derive(Debug, Clone)]
pub struct Cgats {
    vendor: Vendor,
    metadata: Vec<String>,
    fields:   Vec<Field>,
    values:   Vec<CgatsValue>,
}

impl Cgats {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Cgats::try_from(File::open(path)?)
    }
}


impl Default for Cgats {
    fn default() -> Self {
        Cgats {
            vendor: Vendor::Cgats,
            metadata: Vec::new(),
            fields: Vec::new(),
            values: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum CgatsValue {
    Float(f32),
    Int(usize),
    Text(String),
}

impl fmt::Display for CgatsValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CgatsValue::Float(v) => write!(f, "{}", v),
            CgatsValue::Int(v)   => write!(f, "{}", v),
            CgatsValue::Text(v)  => write!(f, "{}", v),
        }
    }
}
