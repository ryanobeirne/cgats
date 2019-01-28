use super::*;
use std::str::FromStr;
use std::fmt;

// Container for what is between BEGIN_DATA_FORMAT and END_DATA_FORMAT
pub type DataFormat = Vec<Field>;

// Known data format types: This list is incomplete
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Field {
    // String
    SAMPLE_ID, SAMPLE_NAME, BLANK,

    // Float
    RGB_R, RGB_G, RGB_B,
    CMYK_C, CMYK_M, CMYK_Y, CMYK_K,
    FIVECLR_1, FIVECLR_2, FIVECLR_3, FIVECLR_4, FIVECLR_5,
    SIXCLR_1, SIXCLR_2, SIXCLR_3, SIXCLR_4, SIXCLR_5, SIXCLR_6,
    SEVENCLR_1, SEVENCLR_2, SEVENCLR_3, SEVENCLR_4, SEVENCLR_5, SEVENCLR_6, SEVENCLR_7,
    EIGHTCLR_1, EIGHTCLR_2, EIGHTCLR_3, EIGHTCLR_4, EIGHTCLR_5, EIGHTCLR_6, EIGHTCLR_7, EIGHTCLR_8,
    D_RED, D_GREEN, D_BLUE, D_VIS,
    LAB_L, LAB_A, LAB_B, LAB_C, LAB_H,
    DE_1976, DE_1994, DE_1994T, DE_CMC, DE_CMC2, DE_2000,
    XYZ_X, XYZ_Y, XYZ_Z,
    XYY_X, XYY_Y, XYY_CAPY,
    SPECTRAL_340, SPECTRAL_350, SPECTRAL_360, SPECTRAL_370, SPECTRAL_380,
    SPECTRAL_390, SPECTRAL_400, SPECTRAL_410, SPECTRAL_420, SPECTRAL_430,
    SPECTRAL_440, SPECTRAL_450, SPECTRAL_460, SPECTRAL_470, SPECTRAL_480,
    SPECTRAL_490, SPECTRAL_500, SPECTRAL_510, SPECTRAL_520, SPECTRAL_530,
    SPECTRAL_540, SPECTRAL_550, SPECTRAL_560, SPECTRAL_570, SPECTRAL_580,
    SPECTRAL_590, SPECTRAL_600, SPECTRAL_610, SPECTRAL_620, SPECTRAL_630,
    SPECTRAL_640, SPECTRAL_650, SPECTRAL_660, SPECTRAL_670, SPECTRAL_680,
    SPECTRAL_690, SPECTRAL_700, SPECTRAL_710, SPECTRAL_720, SPECTRAL_730,
    SPECTRAL_740, SPECTRAL_750, SPECTRAL_760, SPECTRAL_770, SPECTRAL_780,
    SPECTRAL_790, SPECTRAL_800, SPECTRAL_810, SPECTRAL_820, SPECTRAL_830
}

// Implicit DATA_FORMAT for ColorBurst LinFiles
#[allow(non_snake_case)]
pub fn ColorBurstFormat() -> DataFormat {
    vec![
        Field::D_RED,
        Field::D_GREEN,
        Field::D_BLUE,
        Field::D_VIS,
        Field::LAB_L,
        Field::LAB_A,
        Field::LAB_B,
    ]
}

impl Field {
    pub fn is_float(&self) -> bool {
        use Field::*;
        match &self {
            SAMPLE_NAME => false,
            SAMPLE_ID => false,
            BLANK => false,
            _ => true
        }
    }

}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = format!("{:?}", self)
            .replace("FIVE",  "5")
            .replace("SIX",   "6")
            .replace("SEVEN", "7")
            .replace("EIGHT", "8");

        write!(f, "{}", result)
    }
}

impl FromStr for Field {
    type Err = CgatsError;

    fn from_str(s: &str) -> CgatsResult<Self> {
        use Field::*;
        match s.to_uppercase().as_ref() {
            "SAMPLE_ID"   | "SAMPLEID" | "SAMPLE" => Ok(SAMPLE_ID),
            "SAMPLE_NAME" | "SAMPLENAME" => Ok(SAMPLE_NAME),
            "" | "BLANK" => Ok(BLANK),

            "RGB_R"   => Ok(RGB_R),
            "RGB_G"   => Ok(RGB_G),
            "RGB_B"   => Ok(RGB_B),
            
            "CMYK_C"  => Ok(CMYK_C),
            "CMYK_M"  => Ok(CMYK_M),
            "CMYK_Y"  => Ok(CMYK_Y),
            "CMYK_K"  => Ok(CMYK_K),

            "5CLR_1" => Ok(FIVECLR_1),
            "5CLR_2" => Ok(FIVECLR_2),
            "5CLR_3" => Ok(FIVECLR_3),
            "5CLR_4" => Ok(FIVECLR_4),
            "5CLR_5" => Ok(FIVECLR_5),
    
            "6CLR_1" => Ok(SIXCLR_1),
            "6CLR_2" => Ok(SIXCLR_2),
            "6CLR_3" => Ok(SIXCLR_3),
            "6CLR_4" => Ok(SIXCLR_4),
            "6CLR_5" => Ok(SIXCLR_5),
            "6CLR_6" => Ok(SIXCLR_6),

            "7CLR_1" => Ok(SEVENCLR_1),
            "7CLR_2" => Ok(SEVENCLR_2),
            "7CLR_3" => Ok(SEVENCLR_3),
            "7CLR_4" => Ok(SEVENCLR_4),
            "7CLR_5" => Ok(SEVENCLR_5),
            "7CLR_6" => Ok(SEVENCLR_6),
            "7CLR_7" => Ok(SEVENCLR_7),

            "8CLR_1" => Ok(EIGHTCLR_1),
            "8CLR_2" => Ok(EIGHTCLR_2),
            "8CLR_3" => Ok(EIGHTCLR_3),
            "8CLR_4" => Ok(EIGHTCLR_4),
            "8CLR_5" => Ok(EIGHTCLR_5),
            "8CLR_6" => Ok(EIGHTCLR_6),
            "8CLR_7" => Ok(EIGHTCLR_7),
            "8CLR_8" => Ok(EIGHTCLR_8),

            "LAB_L"   => Ok(LAB_L),
            "LAB_A"   => Ok(LAB_A),
            "LAB_B"   => Ok(LAB_B),
            "LAB_C"   => Ok(LAB_C),
            "LAB_H"   => Ok(LAB_H),
            "XYZ_X"   => Ok(XYZ_X),
            "XYZ_Y"   => Ok(XYZ_Y),
            "XYZ_Z"   => Ok(XYZ_Z),
            "XYY_X"   => Ok(XYY_X),
            "XYY_Y"   => Ok(XYY_Y),
            "XYY_CAPY" => Ok(XYY_CAPY),

            "D_RED"   => Ok(D_RED),
            "D_GREEN" => Ok(D_GREEN),
            "D_BLUE"  => Ok(D_BLUE),
            "D_VISUAL" | "D_VIS" => Ok(D_VIS),

            "LAB_DE" | "DE1976" | "DE76" | "DE" | "DE_76" | "DE_1976"
                => Ok(DE_1976),

            "LAB_DE_1994" | "DE1994" | "DE94" | "DE1994G" | "DE94G" | "DE94_G" |
            "DE_1994" | "DE_94" | "DE_1994G" | "DE_1994_G" | "DE_94G" | "DE_94_G"
                => Ok(DE_1994),

            "LAB_DE_1994T" | "DE1994T" | "DE94T" | "DE_1994_T" | "DE_94T" |
            "DE_1994T" | "DE_94_T" | "DE94_T"
                => Ok(DE_1994T),

            "LAB_DE_CMC" | "DECMC" | "DECMC1" | "CMC" | "CMC 1:1" | "DE_CMC1" |
            "DE_CMC_1" | "DE_CMC"
                => Ok(DE_CMC),

            "LAB_DE_CMC2" | "DECMC2" | "CMC2" | "CMC 2:1" |
            "DE_CMC_2" | "DE_CMC2" | "DE_CMC 2:1" | "DECMC_2"
                => Ok(DE_CMC2),
            
            "LAB_DE_2001" | "DE2000" | "DE00" | "DE_2000" | "DE_00"
                => Ok(DE_2000),

            "SPECTRAL_340" => Ok(SPECTRAL_340),
            "SPECTRAL_350" => Ok(SPECTRAL_350),
            "SPECTRAL_360" => Ok(SPECTRAL_360),
            "SPECTRAL_370" => Ok(SPECTRAL_370),
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
            "SPECTRAL_790" => Ok(SPECTRAL_790),
            "SPECTRAL_800" => Ok(SPECTRAL_800),
            "SPECTRAL_810" => Ok(SPECTRAL_810),
            "SPECTRAL_820" => Ok(SPECTRAL_820),
            "SPECTRAL_830" => Ok(SPECTRAL_830),
            _ => Err(CgatsError::UnknownFormatType)
        }
    }

}