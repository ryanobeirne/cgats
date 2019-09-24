use super::*;

use std::str::FromStr;
use std::fmt;

const KEYWORDS: &[&str] = &["argyll", "cti1", "cgats", "colorburst", "curve"];

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Vendor {
    Argyll,
    Cgats,
    ColorBurst,
    Curve,
    Other(String),
}

impl<'a> FromStr for Vendor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Vendor> {
        if s.is_empty() {
            return Err(Error::UnknownVendor);
        }

        let s_lower = s.to_lowercase();

        for keyword in KEYWORDS.iter() {
            if s_lower.contains(keyword) {
                let vendor = match *keyword {
                    "argyll" | "cti1" => Vendor::Argyll,
                    "cgats"           => Vendor::Cgats,
                    "colorburst"      => Vendor::ColorBurst,
                    "curve"           => Vendor::Curve,
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
    assert_eq!(Vendor::from_str("ColorBurst"), Ok(Vendor::ColorBurst));
    assert_eq!(Vendor::from_str("CGATS.17"), Ok(Vendor::Cgats));
    assert_eq!(Vendor::from_str("File Created by Curve3"), Ok(Vendor::Curve));
    assert_eq!(Vendor::from_str("CTI1"), Ok(Vendor::Argyll));
    assert_eq!(Vendor::from_str("derp"), Ok(Vendor::Other("derp".to_owned())));
    assert_eq!(Vendor::from_str(""), Err(Error::UnknownVendor));
}
