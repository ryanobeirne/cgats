use crate::*;

/// A sample value.
/// Can be an integer, floating point decimal, or text string
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum CgatsValue {
    /// An integer value.
    /// Typically only used for `SAMPLE_ID` field, although other fields are acceptable.
    Int(usize),
    /// A floating point decimal value.
    /// Most values will use this type.
    Float(f32),
    /// A text string value.
    /// Typically only used for `SAMPLE_NAME` field.
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

impl From<String> for CgatsValue {
    fn from(s: String) -> Self  {
        CgatsValue::from(s.as_str())
    }
}

impl From<f32> for CgatsValue {
    fn from(f: f32) -> CgatsValue {
        CgatsValue::Float(f)
    }
}

impl From<usize> for CgatsValue {
    fn from(i: usize) -> CgatsValue {
        CgatsValue::Int(i)
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

// Sample ////////////////////////////////////////////////////////////////////
/// Iterator over Cgats samples
pub struct Samples<'a> {
    pub(crate) len: usize,
    pub(crate) index: usize,
    pub(crate) samples: Vec<&'a Sample>,
}

impl<'a> Iterator for Samples<'a> {
    type Item = &'a Sample;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            self.index += 1;
            Some(self.samples[self.index - 1])
        }
    }
}

impl<'a> FromIterator<&'a Sample> for Samples<'a> {
    fn from_iter<I: IntoIterator<Item=&'a Sample>>(iter: I) -> Self {
        let samples: Vec<&'a Sample> = iter.into_iter().collect(); 

        Samples {
            len: samples.len(),
            index: 0,
            samples,
        }
    }
}

/// A measurement sample complsed of `CgatsValue`s
#[derive(Debug, Clone)]
pub struct Sample {
    pub(crate) values: Vec<CgatsValue>,
}

impl Sample {
    /// Returns the number of values in the sample
    pub fn n_values(&self) -> usize {
        self.values.len()
    }

    /// Returns an interator over the sample values
    pub fn values<'a>(&'a self) -> Values<'a> {
        self.into_iter()
    }

    /// Get a sample value by index.
    /// Panics if the index is out of range.
    pub fn value(&self, index: usize) -> &CgatsValue {
        &self.values[index]
    }

    /// Get a sample value by index.
    pub fn get_value(&self, index: usize) -> Option<&CgatsValue> {
        self.values.get(index)
    }
}

impl<'a> IntoIterator for &'a Sample {
    type Item = &'a CgatsValue;
    type IntoIter = Values<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Values {
            len: self.n_values(),
            index: 0,
            values: self.values.iter().collect(),
        }
    }
}

/// Iterator over CgatsValues
pub struct Values<'a> {
    len: usize,
    index: usize,
    values: Vec<&'a CgatsValue>
}

impl<'a> Iterator for Values<'a> {
    type Item = &'a CgatsValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            self.index += 1;
            Some(self.values[self.index - 1])
        } else {
            None
        }
    }
}

impl<'a> FromIterator<CgatsValue> for Sample {
    fn from_iter<I: IntoIterator<Item=CgatsValue>>(iter: I) -> Self {
        Sample {
            values: iter.into_iter().collect(),
        }
    }
}

impl<'a> FromIterator<&'a str> for Sample {
    fn from_iter<I: IntoIterator<Item=&'a str>>(iter: I) -> Self {
        Sample {
            values: iter.into_iter().map(|s| CgatsValue::from(s)).collect(),
        }
    }
}
