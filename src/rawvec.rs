use super::*;
use std::io::{BufReader, BufRead};

pub type RawVec = Vec<Vec<String>>;

// Get the CgatsType from the first line in the RawVec (first line in file)
pub fn get_cgats_type(raw_vec: &RawVec) -> Option<CgatsType> {
    let mut s = String::new();

    // Push the first line into a single string
    for item in raw_vec[0].iter() {
        s.push_str(&item.to_lowercase());
    }

    // Search the string for a CgatsType
    CgatsType::from(&s)
}

// Read a file into a Vector of a Vector of lines (RawVec)
pub fn read_file_to_raw_vec<T: AsRef<Path>>(raw_vec: &mut RawVec, file: T) -> CgatsResult<()> {
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

// Extract metadata from CGATS file: anything that is not between bookends:
// e.g. BEGIN_DATA_FORMAT...END_DATA_FORMAT // BEGIN_DATA...END_DATA
pub fn extract_meta_data(raw_vec: &RawVec) -> Option<RawVec> {
    // No sense in doing anything if there's nothing here
    if raw_vec.len() < 1 {
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

    Some(meta_vec)
}

// Extract the DATA_FORMAT into a Vector of DataFormatTypes (DataFormat)
pub fn extract_data_format(raw_vec: &RawVec) -> CgatsResult<DataFormat> {
    // We need at least 2 lines to extract DATA_FORMAT
    // OK, really 3 lines, but we only need to see 2
    if raw_vec.len() < 2 {
        return Err(CgatsError::NoDataFormat);
    }

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
pub fn extract_data(raw_vec: &RawVec) -> CgatsResult<RawVec> {
    // We need at least 3 lines to define DATA
    if raw_vec.len() < 3 {
        return Err(CgatsError::NoData);
    }

    // Push DATA here
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