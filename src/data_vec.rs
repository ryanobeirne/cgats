use super::*;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct DataLine {
    pub raw_samples: Vec<String>
}

impl DataLine {
    fn new() -> DataLine {
        DataLine {
            raw_samples: Vec::new()
        }
    }

    fn push(&mut self, string: String) {
        self.raw_samples.push(string)
    }

    pub fn from(raw_samples: Vec<String>) -> DataLine {
        DataLine { raw_samples }
    }

    pub fn insert(&mut self, index: usize, s: &str) {
        self.raw_samples.insert(index, String::from(s))
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct DataVec {
    pub lines: Vec<DataLine>
}

impl DataVec {
    pub fn new() -> DataVec {
        DataVec{
            lines: Vec::new()
        }
    }

    pub fn insert(&mut self, index: usize, s: &str) {
        self.lines.insert(index, DataLine::from(vec![String::from(s)]))
    }

    pub fn from(lines: Vec<DataLine>) -> DataVec {
        DataVec { lines }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> CgatsResult<DataVec> {
        let mut data_vec = DataVec::new();

        // Loop through lines and trim trailing whitespace
        for line in BufReader::new(File::open(path)?).lines() {
            let text = match line {
                Ok(txt) => txt.trim().to_string(),
                Err(_)  => "".to_string()
            };

            // If the file uses carriage returns, split those up as well
            let v_cr = text.split("\r")
                .filter(|l| !l.is_empty())
                .map(|l| l.trim());

            // Push each item in a line into a Vector
            for split_line in v_cr {
                let split = split_line.split("\t");
                let mut v = DataLine::new();

                for item in split {
                    v.push(item.trim().to_string());
                }

                // Push the Vectors into the RawVec
                data_vec.lines.push(v);
            }
        }

        // Make sure the file is not empty
        if data_vec.lines.is_empty() {
            Err(Error::EmptyFile)
        } else {
            Ok(data_vec)
        }
    }

    pub fn to_data_map(&self) -> CgatsResult<DataMap> {
        let mut map = DataMap::new();
        
        let data = self.extract_data()?;

        for (index, line) in data.lines.iter().enumerate() {
            let values = line.raw_samples.iter()
                .map(|val|
                    CgatsValue::from_str(&val).unwrap_or(CgatsValue::default())
                ).collect();
            let sample = Sample  { values };
            map.insert(index, sample);
        }

        Ok(map)
    }

    // Extract the DATA_FORMAT into a Vector of DataFormatTypes (DataFormat)
    pub fn extract_data_format(&self) -> CgatsResult<DataFormat> {
        // We need at least 2 lines to extract DATA_FORMAT
        // OK, really 3 lines, but we only need to see 2
        if self.lines.len() < 2 {
            return Err(Error::NoDataFormat);
        }

        // Use implicit format type for ColorBurst LinFiles
        let vendor = &self.get_vendor();
        if let Some(Vendor::ColorBurst) = vendor {
            return Ok(field::ColorBurstFormat());
        }

        let mut data_format = DataFormat::new();

        // Loop through the RawVec and find the BEGIN_DATA_FORMAT tag
        // then take the next line as a tab-delimited Vector
        for (index, item) in self.lines.iter().enumerate() {
            match item.raw_samples[0].as_str() {
                "BEGIN_DATA_FORMAT" => {
                    for format_type in self.lines[index + 1].raw_samples.iter() {
                        let format = Field::from_str(&format_type)?;
                        data_format.push(format);
                    }
                    break;
                },
                _ => continue
            };
        }

        // Check that the DATA_FORMAT is not empty
        if data_format.len() < 1 {
            Err(Error::NoDataFormat)
        } else {
            Ok(data_format)
        }

    }

    // Extract the data betweeen BEGIN_DATA and END_DATA into a RawVec
    pub fn extract_data(&self) -> CgatsResult<DataVec> {
        // We need at least 3 lines to define DATA
        if self.lines.len() < 3 {
            return Err(Error::NoData);
        }

        // Push DATA here
        let mut data_vec = DataVec::new();

        // Loop through the first item of each line and look for the tags.
        for (index, item) in self.lines.iter().enumerate() {
            match item.raw_samples[0].as_str() {
                "BEGIN_DATA" => {
                    // Loop through each line after BEGIN_DATA and push the next
                    for format_type in self.lines[index + 1..].iter() {
                        data_vec.lines.push(format_type.clone());
                    }
                },
                "END_DATA" => {
                    // Pop the last line off the Vector and stop looking
                    data_vec.lines.pop();
                    break;
                },
                _ => continue
            };
        }

        // Check that we actually found some data
        if data_vec.lines.is_empty() {
            Err(Error::NoData)
        } else {
            Ok(data_vec)
        }

    }

    // Extract metadata from CGATS file: anything that is not between bookends:
    // e.g. BEGIN_DATA_FORMAT...END_DATA_FORMAT // BEGIN_DATA...END_DATA
    pub fn extract_meta_data(&self) -> DataVec {
        // Push metadata here
        let mut meta_vec = DataVec::new();

        // Don't push anything between these tags (or the the tags themselves)
        let bookends = &[
            "BEGIN_DATA_FORMAT", "END_DATA_FORMAT",
            "BEGIN_DATA", "END_DATA"
        ];

        // Loop through the raw_vec and toggle pushing to meta_vec
        // based on the presence of bookend tags
        let mut index = 0;
        let mut tag_switch = true;
        while index < self.lines.len() {
            let item = &self.lines[index];
            if bookends.contains(&item.raw_samples[0].as_str()) {
                if tag_switch { tag_switch = false } else { tag_switch = true };
                index += 1;
                continue;
            }
            if tag_switch {
                meta_vec.lines.push(item.clone());
            }
            index += 1;
        }

        meta_vec
    }

    // Get the CgatsType from the first line in the RawVec (first line in file)
    pub fn get_vendor(&self) -> Option<Vendor> {
        let s = self.lines.first()?.raw_samples.iter()
            .map(|s| s.to_lowercase())
            .collect::<String>();

        // Search the string for a CgatsType
        match Vendor::from_str(&s) {
            Ok(cgt) => Some(cgt),
            Err(_) => None,
        }
    }

    pub fn meta_renumber_sets(&mut self, num: usize) {
        for line in self.lines.iter_mut() {
            if line.raw_samples[0].contains("NUMBER_OF_SETS") {
                *line = DataLine::from(vec!["NUMBER_OF_SETS".to_string(), num.to_string()]);
            }
        }
    }
}

impl fmt::Display for DataLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut values = self.raw_samples.iter()
            .map(|line| format!("{}\t", line))
            .collect::<String>();

        values.pop();

        write!(f, "{}", values)
    }
}

impl fmt::Display for DataVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            self.lines.iter()
                .map(|line| format!("{}\n", line))
                .collect::<String>()
        )
    }
}

#[test]
fn from_file() -> CgatsResult<()> {
    let raw = DataVec::from_file("test_files/curve0.txt")?;
    println!("{:?}", raw);
    Ok(())
}