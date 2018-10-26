use super::*;

// Container for what is between BEGIN_DATA_FORMAT and END_DATA_FORMAT
pub type DataFormat = Vec<DataFormatType>;

// Known data format types: This list is incomplete
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DataFormatType {
    // String
    SAMPLE_ID, SAMPLE_NAME, BLANK,

    // f64
    CMYK_C, CMYK_M, CMYK_Y, CMYK_K,
    RGB_R, RGB_G, RGB_B,
    LAB_L, LAB_A, LAB_B, LAB_C, LAB_H,
    LAB_DE, LAB_DE_94, LAB_DE_94T, LAB_DE_CMC, LAB_DE2000,
    XYZ_X, XYZ_Y, XYZ_Z,
    XYY_X, XYY_Y, XYY_CAPY,
    D_RED, D_GREEN, D_BLUE, D_VIS,
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

// Implicit DATA_FORMAT for ColorBurst LinFiles
#[allow(non_snake_case)]
pub fn ColorBurstFormat() -> DataFormat {
    vec![
        DataFormatType::D_RED,
        DataFormatType::D_GREEN,
        DataFormatType::D_BLUE,
        DataFormatType::D_VIS,
        DataFormatType::LAB_L,
        DataFormatType::LAB_A,
        DataFormatType::LAB_B,
    ]
}

impl DataFormatType {
    pub fn display(&self) -> String {
        format!("{}", &self)
    }

    // Convert a &str to a DataFormatType
    pub fn from(s: &str) -> CgatsResult<Self> {
        use DataFormatType::*;
        match s.to_uppercase().as_ref() {
            "SAMPLE_ID" | "SAMPLEID" | "SAMPLE" => Ok(SAMPLE_ID),
            "SAMPLE_NAME" | "SAMPLENAME" => Ok(SAMPLE_NAME),
	        ""        => Ok(BLANK),
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
            "LAB_C"   => Ok(LAB_C),
            "LAB_H"   => Ok(LAB_H),
            "XYZ_X"   => Ok(XYZ_X),
            "XYZ_Y"   => Ok(XYZ_Y),
            "XYZ_Z"   => Ok(XYZ_Z),
            "D_RED"   => Ok(D_RED),
            "D_GREEN" => Ok(D_GREEN),
            "D_BLUE"  => Ok(D_BLUE),
            "D_VISUAL" | "D_VIS" => Ok(D_VIS),
            "LAB_DE"       => Ok(LAB_DE),
            "LAB_DE_94"    => Ok(LAB_DE_94),
            "LAB_DE_CMC"   => Ok(LAB_DE_CMC),
            "LAB_DE_2000"  => Ok(LAB_DE2000),
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