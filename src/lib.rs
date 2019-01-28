mod cgats;
mod vendor;
mod field;
mod data_map;
mod data_vec;
mod error;
mod compare;

#[cfg(test)]
mod test;

pub use self::cgats::Cgats;
pub use compare::CgatsVec;
pub use error::{CgatsResult, CgatsError};
use vendor::Vendor;
use field::*;
use data_map::*;
use data_vec::*;
