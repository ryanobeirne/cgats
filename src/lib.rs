use std::fmt;
use std::iter::FromIterator;
use std::path::Path;
use std::fs::File;
use std::convert::TryFrom;

pub mod field;
pub mod read;
pub mod vendor;
#[macro_use]
mod error;
mod sample;

use field::*;
use vendor::*; 
use error::*;
use sample::*;

pub use error::*;

/// The CGATS object type.
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

    /// Get a sample by index
    pub fn get_sample(&self, index: usize) -> Option<&Sample> {
        self.samples.get(index)
    }

    /// Get a value from sample x field indexes.
    /// Panics if the indexes are out of range.
    pub fn value(&self, sample: usize, field: usize) -> &CgatsValue {
        &self.samples[sample].values[field]
    }

    /// Get a `CgatsValue` by sample index and field index
    pub fn get_value(&self, sample: usize, field: usize) -> Option<&CgatsValue> {
        self.samples.get(sample)?.get_value(field)
    }

    /// Get the number of samples in the set
    pub fn n_samples(&self) -> usize {
        self.samples.len()
    }

    /// Get the number of fields in the set
    pub fn n_fields(&self) -> usize {
        self.fields.len()
    }

    /// Returns an iterator over the Samples
    pub fn samples<'a>(&'a self) -> Samples<'a> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a Cgats {
    type Item = &'a Sample;
    type IntoIter = Samples<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Samples {
            len: self.n_samples(),
            index: 0,
            samples: self.samples.iter().collect(),
        }
    }
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

#[test]
fn cgats_get_value() -> Result<()> {
    let cgats = Cgats::from_file("test_files/cgats1.tsv")?;
    let value = cgats.value(4, 1);
    assert_eq!(value, &CgatsValue::from("Blue"));
    Ok(())
}
