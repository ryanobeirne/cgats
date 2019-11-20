use super::*;

use std::str::FromStr;
use std::fmt;

const KEYWORDS: &[&str] = &["argyll", "cti1", "cgats", "colorburst", "curve", "xrite", "x-rite", "i1", "profiler"];

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Vendor {
    Argyll,
    Cgats,
    ColorBurst,
    Curve,
    Xrite,
    Other(String),
}

impl Vendor {
    pub fn new(s: &'static str) -> Self {
        match Vendor::from_str(s) {
            Ok(vendor) => vendor,
            Err(_) => Vendor::Other(s.to_owned()),
        }
    }
}

impl From<&str> for Vendor {
    fn from(s: &str) -> Vendor {
        match Vendor::from_str(s) {
            Ok(vendor) => vendor,
            Err(_) => Vendor::Cgats,
        }
    }
}

impl FromStr for Vendor {
    type Err = BoxError;
    fn from_str(s: &str) -> Result<Vendor> {
        let s = s.trim();
        if s.is_empty() {
            return boxerr!(Error::UnknownVendor);
        }

        for keyword in KEYWORDS.iter() {
            if s.to_lowercase().contains(keyword) {
                let vendor = match *keyword {
                    "argyll" | "cti1" => Vendor::Argyll,
                    "cgats"           => Vendor::Cgats,
                    "colorburst"      => Vendor::ColorBurst,
                    "curve"           => Vendor::Curve,
                    "xrite" | "x-rite" | "i1" | "profiler" => Vendor::Xrite,
                    _ => unreachable!("Vendor keyword not in list! [vendor::KEYWORDS]"),
                };

                return Ok(vendor);
            }
        }

        Ok(Vendor::Other(s.to_owned()))
    }
}

impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[test]
fn from_str() {
    assert_eq!(Vendor::from_str("ColorBurst").unwrap(), Vendor::ColorBurst);
    assert_eq!(Vendor::from_str("CGATS.17").unwrap(), Vendor::Cgats);
    assert_eq!(Vendor::from_str("File Created by Curve3").unwrap(), Vendor::Curve);
    assert_eq!(Vendor::from_str("CTI1").unwrap(), Vendor::Argyll);
    assert_eq!(Vendor::from_str("derp").unwrap(), Vendor::Other("derp".to_owned()));
    assert!(Vendor::from_str("").is_err());
}
