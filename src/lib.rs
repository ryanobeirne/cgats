use std::fs::File;
use std::path::Path;
use std::fmt;
use std::collections::BTreeMap;

pub mod error;
pub use error::*;

pub mod format;
pub use format::*;

#[cfg(test)]
mod tests;

pub mod rawvec;
use rawvec::*;

// The meat and potatoes of this crate
#[derive(Debug, Clone)]
pub struct CgatsObject {
    pub raw_vec: RawVec,
    pub cgats_type: Option<CgatsType>,
    pub data_map: CgatsMap,
}

impl CgatsObject {
    pub fn new() -> Self {
        Self {
            raw_vec: RawVec::new(),
            cgats_type: None,
            data_map: CgatsMap::new(),
        }
    }

    // New empty CgatsObject of a given CgatsType
    pub fn new_with_type(cgats_type: CgatsType) -> Self {
        let mut cgo = Self::new();
        cgo.cgats_type = Some(cgats_type);
        cgo
    }

    pub fn len(&self) -> CgatsResult<usize> {
        Ok(self.data()?.len())
    }

    // New CgatsObject from a file
    pub fn from_file<T: AsRef<Path>>(file: T) -> CgatsResult<Self> {
        // Read file into a RawVec
        let mut raw_vec = RawVec::new();
        read_file_to_raw_vec(&mut raw_vec, file)?;

        CgatsObject::from_raw_vec(raw_vec)
    }

    fn from_raw_vec(raw_vec: RawVec) -> CgatsResult<Self> {
        // Determine the CgatsType from the first line of the file
        let cgats_type = get_cgats_type(&raw_vec);
        let data_format = extract_data_format(&raw_vec)?;

        // Validate that the data format and the data have the same item count
        for line in extract_data(&raw_vec)? {
            if line.len() != data_format.len() {
                return Err(CgatsError::FormatDataMismatch);
            } 
        }

        let data_map = CgatsMap::from_raw_vec(&raw_vec)?;

        Ok(Self{raw_vec, cgats_type, data_map})
    }

    pub fn metadata(&self) -> CgatsResult<RawVec> {
        extract_meta_data(&self.raw_vec)
    }

    pub fn data(&self) -> CgatsResult<RawVec> {
        extract_data(&self.raw_vec)
    }

    pub fn data_format(&self) -> CgatsResult<DataFormat> {
        extract_data_format(&self.raw_vec)
    }

    pub fn print_data_format(&self) -> CgatsResult<String> {
        let mut s = String::new();

        // Print DATA_FORMAT
        s.push_str("BEGIN_DATA_FORMAT\n");
        let data_format = &self.data_format()?;
        for (index, format) in data_format.iter().enumerate() {
            s.push_str(&format.display());
            if index == data_format.len() - 1 {
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

    pub fn print_meta_data(&self) -> CgatsResult<String> {
        let mut s = String::new();

        // Print metadata
        let metadata = &self.metadata()?;
        for line in metadata {
            for (index, item) in line.iter().enumerate() {
                s.push_str(item);
                if index == line.len() - 1 {
                    s.push('\n');
                } else {
                    s.push('\t');
                }
            }
        }

        Ok(s)
    }

    pub fn print(&self) -> CgatsResult<String> {
        let mut s = String::new();

        s.push_str(&self.print_meta_data()?);
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
        
        let format = format!("{}({}){:?}", cgt, &self.len()?, &self.data_format()?);

        write!(f, "{}", format)
    }
}

// Extract metadata from CGATS file: anything that is not between bookends:
// e.g. BEGIN_DATA_FORMAT...END_DATA_FORMAT // BEGIN_DATA...END_DATA
fn extract_meta_data(raw_vec: &RawVec) -> CgatsResult<RawVec> {
    let mut meta_vec = RawVec::new();

    let bookends = &[
        "BEGIN_DATA_FORMAT", "END_DATA_FORMAT",
        "BEGIN_DATA", "END_DATA"
    ];

    let mut index = 0;
    let mut tag_switch = true;
    while index < raw_vec.len() {
        let item = &raw_vec[index];
        if bookends.contains(&item[0].as_str()) {
            if tag_switch { tag_switch = false } else { tag_switch = true };
            index += 1;
            continue;
        }
        if tag_switch {
            meta_vec.push(item.clone());
        }
        index += 1;
    }

    Ok(meta_vec)
}

// BTreeMap of CGATS Data
pub type DataMap = BTreeMap<(usize, DataFormatType), String>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CgatsMap(pub DataMap);

impl CgatsMap {
    pub fn new() -> Self {
        let data_map: DataMap = BTreeMap::new();
        CgatsMap(data_map)
    }

    fn from_raw_vec(raw_vec: &RawVec) -> CgatsResult<Self> {
        let mut data_map: DataMap = BTreeMap::new();
        
        let data_format = extract_data_format(&raw_vec)?;
        let data = extract_data(&raw_vec)?;

        for (line_index, line) in data.iter().enumerate() {
            for (index, format) in data_format.iter().enumerate() {
                data_map.insert(
                    (line_index, *format),
                    line[index].clone()
                );
            }
        }

        Ok(CgatsMap(data_map))
    }

    pub fn from_file<T: AsRef<Path>>(file: T) -> CgatsResult<Self> {
        let mut raw_vec = RawVec::new();
        read_file_to_raw_vec(&mut raw_vec, file)?;

        Self::from_raw_vec(&raw_vec)
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