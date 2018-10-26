use std::fs::File;
// use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt;

pub mod error;
use error::*;

pub mod format;
use format::*;

#[cfg(test)]
mod tests;

type RawVec = Vec<Vec<String>>;

#[derive(Debug, Clone)]
pub struct DataColumn<'a, T> {
    header: &'a str,
    data: Vec<T>
}

#[derive(Debug, Clone)]
pub struct DataSet<'a, T: 'a> {
    pub columns: Vec<&'a DataColumn<'a, T>>
}

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

#[derive(Debug, Clone)]
pub struct CgatsObject {
    raw_data: RawVec,
    pub cgats_type: Option<CgatsType>,
    pub format: DataFormat,
    pub data: RawVec,
}

impl<'a> CgatsObject {
    pub fn new() -> Self {
        Self {
            raw_data: Vec::new(),
            cgats_type: None,
            format: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn from_file(file: &'a str) -> CgatsResult<Self> {
        let mut raw_data: RawVec = Vec::new();
        read_file_to_raw_vec(&mut raw_data, file)?;

        let cgats_type = get_cgats_type(&raw_data);

        let format = match cgats_type {
            Some(CgatsType::ColorBurst) => format::ColorBurstFormat(),
            _ => extract_data_format(&raw_data)?,
        };

        let data = extract_data(&raw_data)?;

        for line in &data {
            if line.len() != format.len() {
                return Err(CgatsError::FormatDataMismatch);
            } 
        }

        let cgo = Self {
            raw_data,
            cgats_type,
            format,
            data,
        };

        Ok(cgo)
    }

}

fn get_cgats_type(raw_vec: &RawVec) -> Option<CgatsType> {
    let mut s = String::new();

    for item in raw_vec[0].iter() {
        s.push_str(&item.to_lowercase());
    }

    CgatsType::from(&s)
}

fn read_file_to_raw_vec<'a>(raw_vec: &'a mut RawVec, file: &'a str) -> CgatsResult<()> {
    let f = File::open(file)?;

    for line in BufReader::new(f).lines() {
        let text = match line {
            Ok(txt) => txt.trim().to_string(),
            Err(_)  => "".to_string()
        };

        let cr_split = text.split("\r");

        let mut v_cr:Vec<&str> = Vec::new();

        for cr_line in cr_split {
            if ! cr_line.is_empty() {
                v_cr.push(cr_line.trim());
            }
        }

        for split_line in v_cr {
            let split = split_line.split("\t");
            let mut v: Vec<String> = Vec::new();

            for item in split {
                v.push(item.trim().to_string());
            }

            raw_vec.push(v);
        }
    } 

    if raw_vec.len() < 1 {
        Err(CgatsError::EmptyFile)
    } else {
        Ok(())
    }
}

fn extract_data_format<'a>(raw_vec: &'a RawVec) -> CgatsResult<DataFormat> {
    let mut cgv: DataFormat = Vec::new();

    for (index, item) in raw_vec.iter().enumerate() {
        match item[0].as_str() {
            "BEGIN_DATA_FORMAT" => {
                for format_type in raw_vec[index + 1].iter() {
                    let format = DataFormatType::from(format_type)?;
                    cgv.push(format);
                }
                break;
            },
            _ => continue
        };
    }

    if cgv.len() < 1 {
        Err(CgatsError::NoDataFormat)
    } else {
        Ok(cgv)
    }

}

fn extract_data<'a>(raw_vec: &'a RawVec) -> CgatsResult<RawVec> {
    let mut cgv: RawVec = Vec::new();

    for (index, item) in raw_vec.iter().enumerate() {
        match item[0].as_str() {
            "BEGIN_DATA" => {
                for format_type in raw_vec[index + 1..].iter() {
                    cgv.push(format_type.to_vec());
                }
            },
            "END_DATA" => {
                cgv.pop();
                break;
            },
            _ => continue
        };
    }

    if cgv.len() < 1 {
        Err(CgatsError::NoData)
    } else {
        Ok(cgv)
    }

}