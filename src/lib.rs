use std::fs::File;
// use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::error::Error;
use std::fmt;

pub mod error;
use error::*;

#[cfg(test)]
mod tests;

pub type CgatsVec = Vec<Vec<String>>;

#[derive(Debug, Clone)]
pub struct DataColumn<'a, T> {
    header: &'a str,
    data: Vec<T>
}

#[derive(Debug, Clone)]
pub struct DataSet<'a, T: 'a> {
    pub column: Vec<&'a DataColumn<'a, T>>
}

pub enum CgatsType {
    Cgats,
    ColorBurst,
    Curve,
}

pub struct CgatsObject {
    pub raw_data: CgatsVec,
    pub cgats_type: Option<CgatsType>,
}

impl CgatsObject {
    pub fn new(data: &CgatsVec) -> Self {
        Self {
            raw_data: data.to_vec(),
            cgats_type: None,
        }
    }

    pub fn extract_data(&self) -> CgatsResult<CgatsVec> {
        let mut cgv: CgatsVec = Vec::new();
        for item in &self.raw_data {
            if item[0] == "BEGIN_DATA" {
                cgv.push(item.to_vec());
            } else if item[0] == "END_DATA" {
                break;
            }
        }
        Ok(cgv)
    }
}

#[derive(Debug, PartialEq)]
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

    pub fn from(s: &str) -> Option<Self> {
        use DataFormatType::*;
        match s.to_lowercase().as_ref() {
            "sample_id" | "sampleid" | "sample" => Some(SampleID),
            "sample_name" | "samplename" => Some(SampleName),
            "cmyk_c"  => Some(CmykC),
            "cmyk_m"  => Some(CmykM),
            "cmyk_y"  => Some(CmykY),
            "cmyk_k"  => Some(CmykK),
            "rgb_r"   => Some(RgbR),
            "rgb_g"   => Some(RgbG),
            "rgb_b"   => Some(RgbB),
            "lab_l"   => Some(LabL),
            "lab_a"   => Some(LabA),
            "lab_b"   => Some(LabB),
            "xyz_x"   => Some(XyzX),
            "xyz_y"   => Some(XyzY),
            "xyz_z"   => Some(XyzZ),
            "d_red"   => Some(DRed),
            "d_green" => Some(DGreen),
            "d_blue"  => Some(DBlue),
            "d_visual" | "d_vis" => Some(DVisual),
            "spectral_380" => Some(Spectral380),
            "spectral_390" => Some(Spectral390),
            "spectral_400" => Some(Spectral400),
            "spectral_410" => Some(Spectral410),
            "spectral_420" => Some(Spectral420),
            "spectral_430" => Some(Spectral430),
            "spectral_440" => Some(Spectral440),
            "spectral_450" => Some(Spectral450),
            "spectral_460" => Some(Spectral460),
            "spectral_470" => Some(Spectral470),
            "spectral_480" => Some(Spectral480),
            "spectral_490" => Some(Spectral490),
            "spectral_500" => Some(Spectral500),
            "spectral_510" => Some(Spectral510),
            "spectral_520" => Some(Spectral520),
            "spectral_530" => Some(Spectral530),
            "spectral_540" => Some(Spectral540),
            "spectral_550" => Some(Spectral550),
            "spectral_560" => Some(Spectral560),
            "spectral_570" => Some(Spectral570),
            "spectral_580" => Some(Spectral580),
            "spectral_590" => Some(Spectral590),
            "spectral_600" => Some(Spectral600),
            "spectral_610" => Some(Spectral610),
            "spectral_620" => Some(Spectral620),
            "spectral_630" => Some(Spectral630),
            "spectral_640" => Some(Spectral640),
            "spectral_650" => Some(Spectral650),
            "spectral_660" => Some(Spectral660),
            "spectral_670" => Some(Spectral670),
            "spectral_680" => Some(Spectral680),
            "spectral_690" => Some(Spectral690),
            "spectral_700" => Some(Spectral700),
            "spectral_710" => Some(Spectral710),
            "spectral_720" => Some(Spectral720),
            "spectral_730" => Some(Spectral730),
            "spectral_740" => Some(Spectral740),
            "spectral_750" => Some(Spectral750),
            "spectral_760" => Some(Spectral760),
            "spectral_770" => Some(Spectral770),
            "spectral_780" => Some(Spectral780),
            _ => None
        }
    }
}

impl fmt::Display for DataFormatType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

pub fn read_file_to_cgats_vec(cgd: &mut Vec<Vec<String>>, file: &str) -> std::io::Result<()> {
    let f = File::open(file)?;
    for line in BufReader::new(f).lines() {
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

