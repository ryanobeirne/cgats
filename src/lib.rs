use std::fs::File;
// use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt;

pub mod error;
use error::*;

#[cfg(test)]
mod tests;

type RawVec = Vec<Vec<String>>;
type DataFormat = Vec<DataFormatType>;

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

    pub fn from_file(file: &'a str) -> error::CgatsResult<Self> {
        let mut raw_data: RawVec = Vec::new();
        read_file_to_raw_vec(&mut raw_data, file)?;

        let format = extract_data_format(&raw_data)?;
        let data = extract_data(&raw_data)?;

        let cgo = Self {
            raw_data,
            cgats_type: None,
            format,
            data,
        };

        Ok(cgo)
    }

}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum DataFormatType {
    SAMPLE_ID, SAMPLE_NAME,
    CMYK_C, CMYK_M, CMYK_Y, CMYK_K,
    RGB_R, RGB_G, RGB_B,
    LAB_L, LAB_A, LAB_B,
    XYZ_X, XYZ_Y, XYZ_Z,
    D_RED, D_GREEN, D_BLUE, D_VISUAL,
    SPECTRAL_380, SPECTRAL_390, SPECTRAL_400, SPECTRAL_410, SPECTRAL_420,
    SPECTRAL_430, SPECTRAL_440, SPECTRAL_450, SPECTRAL_460, SPECTRAL_470,
    SPECTRAL_480, SPECTRAL_490, SPECTRAL_500, SPECTRAL_510, SPECTRAL_520,
    SPECTRAL_530, SPECTRAL_540, SPECTRAL_550, SPECTRAL_560, SPECTRAL_570,
    SPECTRAL_580, SPECTRAL_590, SPECTRAL_600, SPECTRAL_610, SPECTRAL_620,
    SPECTRAL_630, SPECTRAL_640, SPECTRAL_650, SPECTRAL_660, SPECTRAL_670,
    SPECTRAL_680, SPECTRAL_690, SPECTRAL_700, SPECTRAL_710, SPECTRAL_720,
    SPECTRAL_730, SPECTRAL_740, SPECTRAL_750, SPECTRAL_760, SPECTRAL_770,
    SPECTRAL_780,
}

impl DataFormatType {
    pub fn display(&self) -> String {
        format!("{}", self)
    }

    pub fn from(s: &str) -> CgatsResult<Self> {
        use DataFormatType::*;
        match s.to_uppercase().as_ref() {
            "SAMPLE_ID" | "SAMPLEID" | "SAMPLE" => Ok(SAMPLE_ID),
            "SAMPLE_NAME" | "SAMPLENAME" => Ok(SAMPLE_NAME),
            "CMYK_C"  => Ok(CMYK_C),
            "CMYK_M"  => Ok(CMYK_M),
            "CMYK_Y"  => Ok(CMYK_Y),
            "CMYK_K"  => Ok(CMYK_K),
            "RGB_R"   => Ok(RGB_R),
            "RGB_G"   => Ok(RGB_G),
            "RGB_B"   => Ok(RGB_B),
            "LAB_L"   => Ok(LAB_L),
            "LAB_A"   => Ok(LAB_A),
            "LAB_B"   => Ok(LAB_B),
            "XYZ_X"   => Ok(XYZ_X),
            "XYZ_Y"   => Ok(XYZ_Y),
            "XYZ_Z"   => Ok(XYZ_Z),
            "D_RED"   => Ok(D_RED),
            "D_GREEN" => Ok(D_GREEN),
            "D_BLUE"  => Ok(D_BLUE),
            "D_VISUAL" | "D_VIS" => Ok(D_VISUAL),
            "SPECTRAL_380" => Ok(SPECTRAL_380),
            "SPECTRAL_390" => Ok(SPECTRAL_390),
            "SPECTRAL_400" => Ok(SPECTRAL_400),
            "SPECTRAL_410" => Ok(SPECTRAL_410),
            "SPECTRAL_420" => Ok(SPECTRAL_420),
            "SPECTRAL_430" => Ok(SPECTRAL_430),
            "SPECTRAL_440" => Ok(SPECTRAL_440),
            "SPECTRAL_450" => Ok(SPECTRAL_450),
            "SPECTRAL_460" => Ok(SPECTRAL_460),
            "SPECTRAL_470" => Ok(SPECTRAL_470),
            "SPECTRAL_480" => Ok(SPECTRAL_480),
            "SPECTRAL_490" => Ok(SPECTRAL_490),
            "SPECTRAL_500" => Ok(SPECTRAL_500),
            "SPECTRAL_510" => Ok(SPECTRAL_510),
            "SPECTRAL_520" => Ok(SPECTRAL_520),
            "SPECTRAL_530" => Ok(SPECTRAL_530),
            "SPECTRAL_540" => Ok(SPECTRAL_540),
            "SPECTRAL_550" => Ok(SPECTRAL_550),
            "SPECTRAL_560" => Ok(SPECTRAL_560),
            "SPECTRAL_570" => Ok(SPECTRAL_570),
            "SPECTRAL_580" => Ok(SPECTRAL_580),
            "SPECTRAL_590" => Ok(SPECTRAL_590),
            "SPECTRAL_600" => Ok(SPECTRAL_600),
            "SPECTRAL_610" => Ok(SPECTRAL_610),
            "SPECTRAL_620" => Ok(SPECTRAL_620),
            "SPECTRAL_630" => Ok(SPECTRAL_630),
            "SPECTRAL_640" => Ok(SPECTRAL_640),
            "SPECTRAL_650" => Ok(SPECTRAL_650),
            "SPECTRAL_660" => Ok(SPECTRAL_660),
            "SPECTRAL_670" => Ok(SPECTRAL_670),
            "SPECTRAL_680" => Ok(SPECTRAL_680),
            "SPECTRAL_690" => Ok(SPECTRAL_690),
            "SPECTRAL_700" => Ok(SPECTRAL_700),
            "SPECTRAL_710" => Ok(SPECTRAL_710),
            "SPECTRAL_720" => Ok(SPECTRAL_720),
            "SPECTRAL_730" => Ok(SPECTRAL_730),
            "SPECTRAL_740" => Ok(SPECTRAL_740),
            "SPECTRAL_750" => Ok(SPECTRAL_750),
            "SPECTRAL_760" => Ok(SPECTRAL_760),
            "SPECTRAL_770" => Ok(SPECTRAL_770),
            "SPECTRAL_780" => Ok(SPECTRAL_780),
            _ => Err(CgatsError::UnknownFormatType)
        }
    }
}

impl fmt::Display for DataFormatType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
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

    Ok(())
}

pub fn extract_data_format<'a>(raw_vec: &'a RawVec) -> CgatsResult<DataFormat> {
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

pub fn extract_data<'a>(raw_vec: &'a RawVec) -> CgatsResult<RawVec> {
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