use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt;
use std::collections::BTreeMap;

pub mod error;
use error::*;

pub mod format;
use format::*;

#[cfg(test)]
mod tests;

pub type RawVec = Vec<Vec<String>>;

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

// The meat and potatoes of this crate
#[derive(Debug, Clone)]
pub struct CgatsObject {
    pub raw_vec: RawVec,
    pub cgats_type: Option<CgatsType>,
    pub metadata: RawVec,
    pub data_format: DataFormat,
    pub data: RawVec,
    pub data_map: CgatsMap,
}

impl CgatsObject {
    pub fn new() -> Self {
        Self {
            raw_vec: RawVec::new(),
            metadata: RawVec::new(),
            cgats_type: None,
            data_format: DataFormat::new(),
            data: RawVec::new(),
            data_map: CgatsMap::new(),
        }
    }

    // New empty CgatsObject of a given CgatsType
    pub fn new_with_type(cgats_type: CgatsType) -> Self {
        let mut cgo = Self::new();
        cgo.cgats_type = Some(cgats_type);
        cgo
    }

    // New CgatsObject from a file
    pub fn from_file<T: AsRef<Path>>(file: T) -> CgatsResult<Self> {
        // Read file into a RawVec
        let mut raw_vec = RawVec::new();
        read_file_to_raw_vec(&mut raw_vec, &file)?;

        CgatsObject::from_raw_vec(raw_vec)
    }

    fn from_raw_vec(raw_vec: RawVec) -> CgatsResult<Self> {
        // Determine the CgatsType from the first line of the file
        let cgats_type = get_cgats_type(&raw_vec);
        let metadata = extract_meta_data(&raw_vec)?;
        let data_format = extract_data_format(&raw_vec)?;

        // Define the data as a vector of vectors of lines
        // between BEGIN_DATA and END_DATA tags
        let data = extract_data(&raw_vec)?;

        // Validate that the data format and the data have the same item count
        for line in &data {
            if line.len() != data_format.len() {
                return Err(CgatsError::FormatDataMismatch);
            } 
        }

        let data_map = CgatsMap::from_raw_vec(&raw_vec)?;

        Ok(Self{raw_vec, cgats_type, metadata, data_format, data, data_map})
    }

    pub fn print_data_format(&self) -> String {
        let mut s = String::new();

        // Print DATA_FORMAT
        s.push_str("BEGIN_DATA_FORMAT\n");
        for (index, format) in self.data_format.iter().enumerate() {
            s.push_str(&format.display());
            if index == self.data_format.len() - 1 {
                s.push('\n');
            } else {
                s.push('\t');
            }
        }
        s.push_str("END_DATA_FORMAT\n");

        s
    }

    pub fn print_data(&self) -> String {
        let mut s = String::new();

        // Print DATA
        s.push_str("BEGIN_DATA\n");
        for line in &self.data {
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

        s
    }
    pub fn print_meta_data(&self) -> String {
        let mut s = String::new();

        // Print metadata
        for line in &self.metadata {
            for (index, item) in line.iter().enumerate() {
                s.push_str(item);
                if index == line.len() - 1 {
                    s.push('\n');
                } else {
                    s.push('\t');
                }
            }
        }

        s
    }


    pub fn print(&self) -> String {
        let mut s = String::new();

        s.push_str(&self.print_meta_data());
        s.push_str(&self.print_data_format());
        s.push_str(&self.print_data());

        s
    }

}

// Get the CgatsType from the first line in the RawVec (first line in file)
fn get_cgats_type(raw_vec: &RawVec) -> Option<CgatsType> {
    let mut s = String::new();

    // Push the first line into a single string
    for item in raw_vec[0].iter() {
        s.push_str(&item.to_lowercase());
    }

    // Search the string for a CgatsType
    CgatsType::from(&s)
}

// Read a file into a Vector of a Vector of lines (RawVec)
fn read_file_to_raw_vec<T: AsRef<Path>>(raw_vec: &mut RawVec, file: T) -> CgatsResult<()> {
    let f = File::open(file)?;

    // Loop through lines and trim trailing whitespace
    for line in BufReader::new(f).lines() {
        let text = match line {
            Ok(txt) => txt.trim().to_string(),
            Err(_)  => "".to_string()
        };

        // If the file uses carriage returns, split those up as well
        let cr_split = text.split("\r");

        // Push each line into a Vector of &str's unless it's a blank line
        let mut v_cr:Vec<&str> = Vec::new();
        for cr_line in cr_split {
            if ! cr_line.is_empty() {
                v_cr.push(cr_line.trim());
            }
        }

        // Push each item in a line into a Vector
        for split_line in v_cr {
            let split = split_line.split("\t");
            let mut v: Vec<String> = Vec::new();

            for item in split {
                v.push(item.trim().to_string());
            }

            // Push the Vectors into the RawVec
            raw_vec.push(v);
        }
    } 

    // Make sure the file is not empty
    if raw_vec.len() < 1 {
        Err(CgatsError::EmptyFile)
    } else {
        Ok(())
    }
}

impl fmt::Display for CgatsObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cgats_type = &self.cgats_type;
        let cgt = match cgats_type {
            Some(cgt) => cgt.display(),
            None => "None".to_string()
        };
        
        let format = format!("{}[{}]{:?}", cgt, &self.data.len(), &self.data_format);

        write!(f, "{:?}", format)
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

// Extract the DATA_FORMAT into a Vector of DataFormatTypes (DataFormat)
fn extract_data_format(raw_vec: &RawVec) -> CgatsResult<DataFormat> {

    // Use implicit format type for ColorBurst LinFiles
    let cgats_type = get_cgats_type(&raw_vec);
    if let Some(CgatsType::ColorBurst) = cgats_type {
        return Ok(format::ColorBurstFormat());
    }

    let mut data_format: DataFormat = Vec::new();

    // Loop through the RawVec and find the BEGIN_DATA_FORMAT tag
    // then take the next line as a tab-delimited Vector
    for (index, item) in raw_vec.iter().enumerate() {
        match item[0].as_str() {
            "BEGIN_DATA_FORMAT" => {
                for format_type in raw_vec[index + 1].iter() {
                    let format = DataFormatType::from(format_type)?;
                    data_format.push(format);
                }
                break;
            },
            _ => continue
        };
    }

    // Check that the DATA_FORMAT is not empty
    if data_format.len() < 1 {
        Err(CgatsError::NoDataFormat)
    } else {
        Ok(data_format)
    }

}

// Extract the data betweeen BEGIN_DATA and END_DATA into a RawVec
fn extract_data(raw_vec: &RawVec) -> CgatsResult<RawVec> {
    let mut data_vec = RawVec::new();

    // Loop through the first item of each line and look for the tags.
    for (index, item) in raw_vec.iter().enumerate() {
        match item[0].as_str() {
            "BEGIN_DATA" => {
                // Loop through each line after BEGIN_DATA and push the next
                for format_type in raw_vec[index + 1..].iter() {
                    data_vec.push(format_type.to_vec());
                }
            },
            "END_DATA" => {
                // Pop the last line off the Vector and stop looking
                data_vec.pop();
                break;
            },
            _ => continue
        };
    }

    // Check that we actually found some data
    if data_vec.len() < 1 {
        Err(CgatsError::NoData)
    } else {
        Ok(data_vec)
    }

}