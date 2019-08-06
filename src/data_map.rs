use super::*;

use std::collections::BTreeMap;
use std::str::FromStr;
use std::fmt;

use deltae::color::LabValue;

pub type DataMap = BTreeMap<usize, Sample>;

// type MapKey = (usize, Field);
pub type Float = f32;

#[derive(Debug, Clone, PartialEq)]
pub struct CgatsValue {
    pub string: String,
    pub float: Option<Float>,
}

impl CgatsValue {
    pub fn from_float(float: Float) -> CgatsValue {
        CgatsValue {
            string: float.to_string(),
            float: Some(float),
        }
    }

    fn add_mut(&mut self, other: &CgatsValue) {
        if let Some(f) = self.float {
                let float = Some(f + other.float.unwrap_or(0.0));
                let string = float.unwrap().to_string();
                self.float = float;
                self.string = string;
        }
    }

    fn divide_mut(&mut self, divisor: usize) {
        if let Some(f) = self.float {
                let float = Some(f / divisor as Float);
                let string = float.unwrap().to_string();
                self.float = float;
                self.string = string;
        }
    }

}

impl FromStr for CgatsValue {
    type Err = Error;
    fn from_str(s: &str) -> Result<CgatsValue> {
        if s.is_empty() {
            return Err(Error::NoData);
        }

        let string = s.to_string();
        let float = s.parse::<Float>().ok();

        Ok(CgatsValue { string, float })
    }
}

impl Default for CgatsValue {
    fn default() -> CgatsValue {
        CgatsValue {
            string: String::new(),
            float: None,
        }
    }
}

impl fmt::Display for CgatsValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.float {
            Some(float) => write!(f, "{}", round_to(float, 4)),
            None => write!(f, "{}", self.string),
        }
    }
}

fn round_to(float: Float, places: i32) -> Float {
    let mult = (10 as Float).powi(places);
    (float * mult).round() / mult
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    pub values: Vec<CgatsValue>
}

impl Sample {
    pub fn add_values(&self, other: &Sample) -> Sample {
        let mut sample = self.clone();

        for (index, value) in sample.values.iter_mut().enumerate() {
            value.add_mut(&other.values[index]);
        }

        sample
    }

    pub fn divide_values(&self, divisor: usize) -> Sample {
        let mut sample = self.clone();

        for value in sample.values.iter_mut() {
            value.divide_mut(divisor)
        }

        sample
    }

    pub fn zero(&self) -> Sample {
        Sample {
            values: self.values.iter().map(|val|
                match val.float {
                    Some(f) => CgatsValue { string: f.to_string(), float: Some(0.0) },
                    None => val.clone(),
                }
            ).collect()
        }
    }

    pub fn to_lab(&self, indexes: &[usize; 3]) -> Option<LabValue> {
        Some(LabValue {
            l: self.values.get(indexes[0])?.float?,
            a: self.values.get(indexes[1])?.float?,
            b: self.values.get(indexes[2])?.float?,
        })
    }
}

impl fmt::Display for Sample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut values = self.values.iter()
            .map(|val| format!("{}\t", val))
            .collect::<String>();

        values.pop();

        write!(f, "{}", values)
    }
}