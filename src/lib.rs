use std::fs::File;
use std::path::Path;
use std::fmt;
use std::collections::BTreeMap;

pub mod rawvec;
use rawvec::*;

pub mod cgatsmap;
use cgatsmap::*;

pub mod compare;

pub mod error;
use error::*;

pub mod format;
use format::*;

#[cfg(test)]
mod tests;

// The meat and potatoes of this crate
#[derive(Debug, Clone)]
pub struct CgatsObject {
    pub raw_vec: RawVec,
    pub cgats_type: Option<CgatsType>,
    pub data_format: DataFormat,
    pub data_map: CgatsMap,
}

impl CgatsObject {
    pub fn new() -> Self {
        Self {
            raw_vec: RawVec::new(),
            cgats_type: None,
            data_format: DataFormat::new(),
            data_map: CgatsMap::new(),
        }
    }

    // New empty CgatsObject of a given CgatsType
    pub fn new_with_type(cgats_type: CgatsType) -> Self {
        let mut cgo = Self::new();
        cgo.cgats_type = Some(cgats_type);
        cgo
    }

    pub fn derive_from(other: &Self) -> Self {
        Self {
            raw_vec: RawVec::new(),
            cgats_type: other.cgats_type.clone(),
            data_format: other.data_format.clone(),
            data_map: CgatsMap::new(),
        }
    }

    pub fn new_with_format(data_format: DataFormat) -> Self {
        let mut cgo = Self::new();
        cgo.data_format = data_format;
        cgo
    }

    pub fn len(&self) -> usize {
        match self.data() {
            Ok(data) => data.len(),
            Err(_) => 0
        }
    }

    // New CgatsObject from a file
    pub fn from_file<T: AsRef<Path>>(file: T) -> CgatsResult<Self> {
        // Read file into a RawVec
        let mut raw_vec = RawVec::new();
        raw_vec.read_file(file)?;

        CgatsObject::from_raw_vec(raw_vec)
    }

    pub fn from_raw_vec(raw_vec: RawVec) -> CgatsResult<Self> {
        // Determine the CgatsType from the first line of the file
        let cgats_type = raw_vec.get_cgats_type();
        let data_format = raw_vec.extract_data_format()?;

        // Validate that the data format and the data have the same item count
        for line in raw_vec.extract_data()?.inner {
            if line.len() != data_format.len() {
                return Err(CgatsError::FormatDataMismatch);
            } 
        }

        let data_map = CgatsMap::from_raw_vec(&raw_vec)?;

        Ok(Self{raw_vec, cgats_type, data_format, data_map})
    }

    pub fn metadata(&self) -> Option<RawVec> {
        self.raw_vec.extract_meta_data()
    }

    pub fn data(&self) -> CgatsResult<RawVec> {
        self.raw_vec.extract_data()
    }

    pub fn print_data_format(&self) -> CgatsResult<String> {
        let mut s = String::new();

        // Print DATA_FORMAT
        s.push_str("BEGIN_DATA_FORMAT\n");
        if self.data_format.len() == 0 {
            return Err(CgatsError::NoDataFormat);
        }
        for (index, format) in self.data_format.iter().enumerate() {
            s.push_str(&format.display());
            if index == self.data_format.len() - 1 {
                s.push('\n');
            } else {
                s.push('\t');
            }
        }
        s.push_str("END_DATA_FORMAT\n");

        Ok(s)
    }

    pub fn print_data(&self) -> CgatsResult<String> {
        let mut s = String::new();

        // Print DATA
        s.push_str("BEGIN_DATA\n");
        let data = &self.data()?;
        if data.len() == 0 {
            return Err(CgatsError::NoData);
        }
        for line in &data.inner {
            for (index, item) in line.iter().enumerate() {
                s.push_str(item);
                if index == line.len() - 1 {
                    s.push('\n');
                } else {
                    s.push('\t');
                }
            }
        }
        s.push_str("END_DATA\n");

        Ok(s)
    }

    pub fn print_meta_data(&self) -> Option<String> {
        let mut s = String::new();

        // Print metadata
        let metadata = &self.metadata()?;
        if metadata.len() == 0 {
            return Some(s);
        }
        for line in &metadata.inner {
            for (index, item) in line.iter().enumerate() {
                s.push_str(item);
                if index == line.len() - 1 {
                    s.push('\n');
                } else {
                    s.push('\t');
                }
            }
        }

        Some(s)
    }

    pub fn print(&self) -> CgatsResult<String> {
        let mut s = String::new();

        if let Some(meta) = &self.print_meta_data() {
            s.push_str(meta);
        }

        s.push_str(&self.print_data_format()?);
        s.push_str(&self.print_data()?);

        Ok(s)
    }

}

impl fmt::Display for CgatsObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cgats_type = &self.cgats_type;
        let cgt = match cgats_type {
            Some(cgt) => cgt.display(),
            None => "None".to_string()
        };
        
        let format = format!("{}({}){:?}", cgt, &self.len(), &self.data_format);

        write!(f, "{}", format)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct CgatsValue {
    pub value: String,
    pub float: f64,
    pub is_float: bool,
}

impl CgatsValue {
    fn from_string(val: &str) -> Self {
        let (value, float, is_float) = match val.parse::<f64>() {
            Ok(f) => ( compare::round_to(f, 4).to_string(), f, true ),
            Err(_) => ( val.to_string(), 0_f64, false )
        };
        Self {value, float, is_float}
    }

    fn from_float(float: f64) -> Self {
        let value = compare::round_to(float, 4).to_string();
        Self { value, float, is_float: true }
    }
}

impl fmt::Display for CgatsValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

// Possible CGATS types with special meanings
#[derive(Debug, Clone)]
pub enum CgatsType {
    Cgats,
    ColorBurst,
    Curve,
}

impl CgatsType {
    pub fn display(&self) -> String {
        format!("{}", &self)
    }

    // Checks if a string contains the the CgatsType name and
    // returns an option of that type
    pub fn from(s: &str) -> Option<Self> {
        use CgatsType::*;
        let types: Vec<Self> = vec![Cgats, ColorBurst, Curve];

        for t in types {
            let cgats_type = t.display().to_lowercase();
            let finder = s.to_lowercase().find(cgats_type.as_str());
            match finder {
                Some(_) => return Some(t),
                None => continue,
            };
        }

       None
    }
}

impl fmt::Display for CgatsType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}