use super::*;
use std::io::{BufReader, BufRead};

pub type DataLine = Vec<String>;
pub type DataSet = Vec<DataLine>;
pub type DataVec =  Vec<DataLine>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RawVec {
    pub inner: DataSet,
}

impl RawVec {
    pub fn new() -> Self {
        Self { inner: DataSet::new() }
    }

    pub fn pop(&mut self) -> Option<DataLine> {
        self.inner.pop()
    }

    pub fn push(&mut self, value: DataLine) {
        self.inner.push(value)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    // Get the CgatsType from the first line in the RawVec (first line in file)
    pub fn get_cgats_type(&self) -> Option<CgatsType> {
        let mut s = String::new();

        // Push the first line into a single string
        for item in self.inner[0].iter() {
            s.push_str(&item.to_lowercase());
        }

        // Search the string for a CgatsType
        match CgatsType::from_str(&s) {
            Ok(cgt) => Some(cgt),
            Err(_) => None,
        }
    }

    pub fn from_file<T: AsRef<Path>>(file: T) -> CgatsResult<Self> {
        let mut raw_vec = Self::new();
        raw_vec.read_file(file)?;
        Ok(raw_vec)
    }

    // Read a file into a Vector of a Vector of lines (RawVec)
    pub fn read_file<T: AsRef<Path>>(&mut self, file: T) -> CgatsResult<()> {
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
                let mut v: DataLine = Vec::new();

                for item in split {
                    v.push(item.trim().to_string());
                }

                // Push the Vectors into the RawVec
                self.push(v);
            }
        } 

        // Make sure the file is not empty
        if self.len() < 1 {
            Err(CgatsError::EmptyFile)
        } else {
            Ok(())
        }
    }

    // Extract metadata from CGATS file: anything that is not between bookends:
    // e.g. BEGIN_DATA_FORMAT...END_DATA_FORMAT // BEGIN_DATA...END_DATA
    pub fn extract_meta_data(&self) -> Option<Self> {
        // No sense in doing anything if there's nothing here
        if self.len() < 1 {
            return None;
        }

        // Push metadata here
        let mut meta_vec = RawVec::new();

        // Don't push anything between these tags (or the the tags themselves)
        let bookends = &[
            "BEGIN_DATA_FORMAT", "END_DATA_FORMAT",
            "BEGIN_DATA", "END_DATA"
        ];

        // Loop through the raw_vec and toggle pushing to meta_vec
        // based on the presence of bookend tags
        let mut index = 0;
        let mut tag_switch = true;
        while index < self.len() {
            let item = &self.inner[index];
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

        Some(meta_vec)
    }

    // Extract the DATA_FORMAT into a Vector of DataFormatTypes (DataFormat)
    pub fn extract_data_format(&self) -> CgatsResult<DataFormat> {
        // We need at least 2 lines to extract DATA_FORMAT
        // OK, really 3 lines, but we only need to see 2
        if self.len() < 2 {
            return Err(CgatsError::NoDataFormat);
        }

        // Use implicit format type for ColorBurst LinFiles
        let cgats_type = &self.get_cgats_type();
        if let Some(CgatsType::ColorBurst) = cgats_type {
            return Ok(format::ColorBurstFormat());
        }

        let mut data_format = DataFormat::new();

        // Loop through the RawVec and find the BEGIN_DATA_FORMAT tag
        // then take the next line as a tab-delimited Vector
        for (index, item) in self.inner.iter().enumerate() {
            match item[0].as_str() {
                "BEGIN_DATA_FORMAT" => {
                    for format_type in self.inner[index + 1].iter() {
                        let format = DataFormatType::from_str(&format_type)?;
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
    pub fn extract_data(&self) -> CgatsResult<Self> {
        // We need at least 3 lines to define DATA
        if self.len() < 3 {
            return Err(CgatsError::NoData);
        }

        // Push DATA here
        let mut data_vec = RawVec::new();

        // Loop through the first item of each line and look for the tags.
        for (index, item) in self.inner.iter().enumerate() {
            match item[0].as_str() {
                "BEGIN_DATA" => {
                    // Loop through each line after BEGIN_DATA and push the next
                    for format_type in self.inner[index + 1..].iter() {
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
}