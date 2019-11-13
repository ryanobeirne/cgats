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

/// vendor: Vendor,
/// metadata: Vec<String>,
/// fields:   Vec<Field>,
/// values:   Vec<CgatsValue>,
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
    Int(usize),
    Float(f32),
    Text(String),
}

impl From<&str> for CgatsValue {
    fn from(s: &str) -> Self {
        if let Ok(i) = s.parse::<usize>() {
            CgatsValue::Int(i)
        } else if let Ok(f) = s.parse::<f32>() {
            CgatsValue::Float(f)
        } else {
            CgatsValue::Text(s.into())
        }
    }
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
