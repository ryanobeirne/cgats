use super::*;

use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Vendor {
    Cgats,
    ColorBurst,
    Curve,
}

impl FromStr for Vendor {
    type Err = CgatsError;

    fn from_str(s: &str) -> CgatsResult<Vendor> {
        use Vendor::*;
        let types: Vec<Vendor> = vec![Cgats, ColorBurst, Curve];

        for t in types.into_iter() {
            let tstring = &t.to_string().to_lowercase();
            if s.to_lowercase().contains(tstring) {
                return Ok(t);
            }
        }

       Err(CgatsError::UnknownVendor)
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
    assert_eq!(Vendor::from_str("derp"), Err(CgatsError::UnknownVendor));
    assert_eq!(Vendor::from_str(""), Err(CgatsError::UnknownVendor));
}