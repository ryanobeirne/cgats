use std::fmt;
use std::iter::FromIterator;
use std::path::Path;
use std::fs::File;
use std::convert::TryFrom;

pub mod field;
pub mod read;
mod vendor;
#[macro_use]
mod error;
mod sample;

use field::*;
use vendor::*; 
use error::*;
use sample::*;

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
    samples:   Vec<Sample>,
}

impl Cgats {
    /// Convert a file path to a CGATS object
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Cgats::try_from(File::open(path)?)
    }

    /// Get the values for a given sample index.
    /// Panics if the index is out of range.
    pub fn sample(&self, index: usize) -> &Sample {
        &self.samples[index]
    }

    /// Get a value from sample x field indexes.
    /// Panics if the indexes are out of range.
    pub fn value(&self, sample: usize, field: usize) -> &CgatsValue {
        &self.samples[sample].values[field]
    }

    /// Get the number of samples in the set
    pub fn n_samples(&self) -> usize {
        self.samples.len()
    }

    /// Get the number of fields in the set
    pub fn n_fields(&self) -> usize {
        self.fields.len()
    }

    pub fn samples<'a>(&'a self) -> Samples<'a> {
        self.samples.iter().collect()
    }
}

#[test]
fn cgats_get_value() -> Result<()> {
    let cgats = Cgats::from_file("test_files/cgats1.tsv")?;
    let value = cgats.value(4, 1);
    assert_eq!(value, &CgatsValue::from("Blue"));
    Ok(())
}

impl Default for Cgats {
    fn default() -> Self {
        Cgats {
            vendor: Vendor::Cgats,
            metadata: Vec::new(),
            fields: Vec::new(),
            samples: Vec::new(),
        }
    }
}
