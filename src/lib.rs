use std::fs::File;
// use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt;

pub mod error;
use error::*;

#[cfg(test)]
mod tests;

pub type RawVec = Vec<Vec<String>>;

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
    pub raw_data: RawVec,
    pub cgats_type: Option<CgatsType>,
}

impl CgatsObject {
    pub fn new(data: &RawVec) -> Self {
        Self {
            raw_data: data.to_vec(),
            cgats_type: None,
        }
    }

    pub fn from_file(file: &str) -> error::CgatsResult<Self> {
        let mut raw_data: RawVec = Vec::new();
        read_file_to_cgats_vec(&mut raw_data, file)?;

        let cgo = Self {
            raw_data,
            cgats_type: None
        };

        Ok(cgo)
    }

    pub fn extract_data_format(&self) -> CgatsResult<Vec<&str>> {
        let mut cgv: Vec<&str> = Vec::new();

        for (index, item) in self.raw_data.iter().enumerate() {
            match item[0].as_str() {
                "BEGIN_DATA_FORMAT" => {
                    for format_type in self.raw_data[index + 1].iter() {
                        cgv.push(format_type);
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

    pub fn extract_data(&self) -> CgatsResult<RawVec> {
        let mut cgv: RawVec = Vec::new();

        for (index, item) in self.raw_data.iter().enumerate() {
            match item[0].as_str() {
                "BEGIN_DATA" => {
                    for format_type in self.raw_data[index + 1..].iter() {
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataFormatType {
    SampleID, SampleName,
    CmykC, CmykM, CmykY, CmykK,
    RgbR, RgbG, RgbB,
    LabL, LabA, LabB,
    XyzX, XyzY, XyzZ,
    DRed, DGreen, DBlue, DVisual,
    Spectral380, Spectral390, Spectral400, Spectral410, Spectral420,
    Spectral430, Spectral440, Spectral450, Spectral460, Spectral470,
    Spectral480, Spectral490, Spectral500, Spectral510, Spectral520,
    Spectral530, Spectral540, Spectral550, Spectral560, Spectral570,
    Spectral580, Spectral590, Spectral600, Spectral610, Spectral620,
    Spectral630, Spectral640, Spectral650, Spectral660, Spectral670,
    Spectral680, Spectral690, Spectral700, Spectral710, Spectral720,
    Spectral730, Spectral740, Spectral750, Spectral760, Spectral770,
    Spectral780,
}

impl DataFormatType {
    pub fn display(&self) -> String {
        format!("{}", self)
    }

    pub fn from(s: &str) -> CgatsResult<Self> {
        use DataFormatType::*;
        match s.to_lowercase().as_ref() {
            "sample_id" | "sampleid" | "sample" => Ok(SampleID),
            "sample_name" | "samplename" => Ok(SampleName),
            "cmyk_c"  => Ok(CmykC),
            "cmyk_m"  => Ok(CmykM),
            "cmyk_y"  => Ok(CmykY),
            "cmyk_k"  => Ok(CmykK),
            "rgb_r"   => Ok(RgbR),
            "rgb_g"   => Ok(RgbG),
            "rgb_b"   => Ok(RgbB),
            "lab_l"   => Ok(LabL),
            "lab_a"   => Ok(LabA),
            "lab_b"   => Ok(LabB),
            "xyz_x"   => Ok(XyzX),
            "xyz_y"   => Ok(XyzY),
            "xyz_z"   => Ok(XyzZ),
            "d_red"   => Ok(DRed),
            "d_green" => Ok(DGreen),
            "d_blue"  => Ok(DBlue),
            "d_visual" | "d_vis" => Ok(DVisual),
            "spectral_380" => Ok(Spectral380),
            "spectral_390" => Ok(Spectral390),
            "spectral_400" => Ok(Spectral400),
            "spectral_410" => Ok(Spectral410),
            "spectral_420" => Ok(Spectral420),
            "spectral_430" => Ok(Spectral430),
            "spectral_440" => Ok(Spectral440),
            "spectral_450" => Ok(Spectral450),
            "spectral_460" => Ok(Spectral460),
            "spectral_470" => Ok(Spectral470),
            "spectral_480" => Ok(Spectral480),
            "spectral_490" => Ok(Spectral490),
            "spectral_500" => Ok(Spectral500),
            "spectral_510" => Ok(Spectral510),
            "spectral_520" => Ok(Spectral520),
            "spectral_530" => Ok(Spectral530),
            "spectral_540" => Ok(Spectral540),
            "spectral_550" => Ok(Spectral550),
            "spectral_560" => Ok(Spectral560),
            "spectral_570" => Ok(Spectral570),
            "spectral_580" => Ok(Spectral580),
            "spectral_590" => Ok(Spectral590),
            "spectral_600" => Ok(Spectral600),
            "spectral_610" => Ok(Spectral610),
            "spectral_620" => Ok(Spectral620),
            "spectral_630" => Ok(Spectral630),
            "spectral_640" => Ok(Spectral640),
            "spectral_650" => Ok(Spectral650),
            "spectral_660" => Ok(Spectral660),
            "spectral_670" => Ok(Spectral670),
            "spectral_680" => Ok(Spectral680),
            "spectral_690" => Ok(Spectral690),
            "spectral_700" => Ok(Spectral700),
            "spectral_710" => Ok(Spectral710),
            "spectral_720" => Ok(Spectral720),
            "spectral_730" => Ok(Spectral730),
            "spectral_740" => Ok(Spectral740),
            "spectral_750" => Ok(Spectral750),
            "spectral_760" => Ok(Spectral760),
            "spectral_770" => Ok(Spectral770),
            "spectral_780" => Ok(Spectral780),
            _ => Err(CgatsError::UnknownDataFormat)
        }
    }
}

impl fmt::Display for DataFormatType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

pub fn read_file_to_cgats_vec(cgd: &mut Vec<Vec<String>>, file: &str) -> CgatsResult<()> {
    let f = File::open(file);

    if let Err(e) = &f {
        eprintln!("{}", e);
        return Err(CgatsError::FileError);
    }

    for line in BufReader::new(f.unwrap()).lines() {
        let text = match line {
            Ok(text) => String::from(text.trim_right()),
            Err(_) => String::from("")
        };

        let cr_split = text.split("\r");

        let mut v_cr:Vec<String> = Vec::new();

        for cr_line in cr_split {
            if ! cr_line.is_empty() {
                v_cr.push(String::from(cr_line.trim_right()));
            }
        }

        for split_line in v_cr {
            let split = split_line.split("\t");
            let mut v: Vec<String> = Vec::new();

            for item in split {
                v.push(String::from(item.trim()));
            }

            cgd.push(v);
        }
    } 

    Ok(())
}

