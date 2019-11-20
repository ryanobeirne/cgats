use crate::*;

/// A sample value
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum CgatsValue {
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
    len: usize,
    index: usize,
    samples: Vec<&'a Sample>,
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

/// A measurement sample
#[derive(Debug, Clone)]
pub struct Sample {
    pub values: Vec<CgatsValue>,
}

impl Sample {
    pub fn n_values(&self) -> usize {
        self.values.len()
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
