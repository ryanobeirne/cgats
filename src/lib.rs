use std::fs::File;
use std::path::Path;
use std::fmt;
use std::collections::BTreeMap;
use std::io::{Write, BufWriter};

mod rawvec;
pub use rawvec::*;

mod cgatsmap;
pub use cgatsmap::*;

mod compare;
pub use compare::*;

mod error;
pub use error::*;

mod format;
pub use format::*;

mod delta;
pub use delta::*;
extern crate deltae;

#[cfg(test)]
mod tests;

// Color measurements deal with very small numbers, so I feel like I should
// be using more accurate floats, but I'm going to stick with f32 for now.
// This type alias will make it easy to change.
type CgatsFloat = f32;

// The meat and potatoes of this crate
#[derive(Debug, Clone, PartialEq)]
pub struct CgatsObject {
    pub raw_vec: RawVec,
    pub cgats_type: Option<CgatsType>,
    pub data_format: DataFormat,
    pub data_map: CgatsMap,
}

impl CgatsObject {
    pub fn new() -> CgatsObject {
        CgatsObject {
            raw_vec: RawVec::new(),
            cgats_type: None,
            data_format: DataFormat::new(),
            data_map: CgatsMap::new(),
        }
    }

    pub fn write_cgats<T: AsRef<Path>>(&self, file: T) -> CgatsResult<()> {
        match File::create(file) {
            Ok(f) => {
                let mut buf = BufWriter::new(f);

                if let Err(e) = buf.write(self.print()?.as_bytes()) {
                    eprintln!("{}", e);
                    return Err(CgatsError::WriteError);
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                return Err(CgatsError::WriteError);
            }
        }
        Ok(())
    }

    // New empty CgatsObject of a given CgatsType
    pub fn new_with_type(cgats_type: CgatsType) -> CgatsObject {
        let mut cgo = CgatsObject::new();
        cgo.cgats_type = Some(cgats_type);
        cgo
    }

    pub fn derive_from(other: &CgatsObject) -> CgatsObject {
        CgatsObject {
            raw_vec: RawVec::new(),
            cgats_type: other.cgats_type.clone(),
            data_format: other.data_format.clone(),
            data_map: CgatsMap::new(),
        }
    }

    pub fn new_with_format(data_format: DataFormat) -> CgatsObject {
        let mut cgo = CgatsObject::new();
        cgo.data_format = data_format;
        cgo
    }

    pub fn len(&self) -> usize {
        self.data_map.0.len() / self.data_format.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data_map.is_empty()
    }

    pub fn map(&mut self) -> CgatsResult<()> {
        if self.data_map.is_empty() {
            return Err(CgatsError::EmptyFile);
        }

        self.data_map = CgatsMap::from_raw_vec(&self.raw_vec)?;
        Ok(())
    }

    pub fn reindex_sample_id(&mut self) {
        self.data_map.reindex_sample_id()
    }

    // New CgatsObject from a file
    pub fn from_file<T: AsRef<Path>>(file: T) -> CgatsResult<CgatsObject> {
        // Read file into a RawVec
        let mut raw_vec = RawVec::new();
        raw_vec.read_file(file)?;

        CgatsObject::from_raw_vec(raw_vec)
    }

    pub fn from_raw_vec(raw_vec: RawVec) -> CgatsResult<CgatsObject> {
        // Determine the CgatsType from the first line of the file
        let cgats_type = raw_vec.get_cgats_type();
        let data_format = raw_vec.extract_data_format()?;

        // Validate that the data format and the data have the same item count
        for line in raw_vec.extract_data()?.0 {
            if line.len() != data_format.len() {
                return Err(CgatsError::FormatDataMismatch);
            } 
        }

        let data_map = CgatsMap::from_raw_vec(&raw_vec)?;

        Ok(CgatsObject{raw_vec, cgats_type, data_format, data_map})
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
            s.push_str(&format.to_string());
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
        let data = &self.data_map.to_data_vec();
        if data.len() == 0 {
            return Err(CgatsError::NoData);
        }

        for line in data {
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
        for line in &metadata.0 {
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

    pub fn is_cb(&self) -> bool {
        self.cgats_type == Some(CgatsType::ColorBurst)
    }

    pub fn has_lab(&self) -> bool {
        self.data_map.has_lab()
    }

    pub fn print(&self) -> CgatsResult<String> {
        let mut s = String::new();

        if let Some(meta) = &self.print_meta_data() {
            s.push_str(meta);
        }

        // Don't print DATA_FORMAT if it's ColorBurst
        if !&self.is_cb() {
            s.push_str(&self.print_data_format()?);
        }

        s.push_str(&self.print_data()?);

        Ok(s)
    }

    pub fn append(&mut self, other: &mut CgatsObject) -> Option<()> {
        let end_data = self.raw_vec.pop();
        if let Some(line) = &end_data {
            if line[0] != "END_DATA".to_string() {
                self.raw_vec.push(line.clone());
            }
        }

        let mut other_data = match other.data() {
            Ok(raw_vec) => raw_vec,
            Err(_) => RawVec::new(),
        };

        self.raw_vec.0.append(&mut other_data.0);

        if let Some(line) = end_data {
            if line[0] == "END_DATA".to_string() {
                self.raw_vec.push(line);
            }
        }

        Some(())
    }

}

impl fmt::Display for CgatsObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cgats_type = &self.cgats_type;
        let cgt = match cgats_type {
            Some(cgt) => cgt.display(),
            None => "None".to_string()
        };

        let data_format = format::fmt_data_format(&self.data_format);
        
        let format = format!("{}({}):{}", cgt, &self.len(), data_format);

        write!(f, "{}", format)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct CgatsValue {
    pub value: String,
    pub float: CgatsFloat,
    pub is_float: bool,
}

impl CgatsValue {
    fn from_string(val: &str) -> CgatsValue {
        let (value, float, is_float) = match val.parse::<CgatsFloat>() {
            Ok(f) => ( compare::round_to(f, 4).to_string(), f, true ),
            Err(_) => ( val.to_string(), 0 as CgatsFloat, false )
        };
        CgatsValue {value, float, is_float}
    }

    fn from_float(float: CgatsFloat) -> CgatsValue {
        let value = compare::round_to(float, 4).to_string();
        CgatsValue { value, float, is_float: true }
    }
}

impl fmt::Display for CgatsValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

// Possible CGATS types with special meanings
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CgatsType {
    Cgats,
    ColorBurst,
    Curve,
}

impl CgatsType {
    pub fn display(&self) -> String {
        format!("{}", &self)
    }

}

impl FromStr for CgatsType {
    type Err = CgatsError;

    fn from_str(s: &str) -> CgatsResult<CgatsType> {
        use CgatsType::*;
        let types: Vec<CgatsType> = vec![Cgats, ColorBurst, Curve];

        for t in types.into_iter() {
            let tstring = &t.display().to_lowercase();
            if s.to_lowercase().contains(tstring) {
                return Ok(t);
            }
        }

       Err(CgatsError::UnknownCgatsType)
    }
}

impl fmt::Display for CgatsType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}