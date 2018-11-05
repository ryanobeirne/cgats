use super::*;

pub type CgatsVec = Vec<CgatsObject>;

// Check that all CgatsObjects have the same data type and sample count
pub fn is_comparable(cgats_vec: &CgatsVec) -> bool {
    let cgo_prime = &cgats_vec[0];
    for object in cgats_vec {
        if cgo_prime.len() != object.len() {
            return false;
        }
    }

    true
}

pub fn average(cgats_vec: &CgatsVec) -> CgatsResult<CgatsObject> {

    match cgats_vec.len() {
        1 => return Ok(cgats_vec[0].clone()),
        0 => return Err(CgatsError::NoData),
        _ => ()
    }

    if !is_comparable(cgats_vec) {
        return Err(CgatsError::CannotCompare);
    } 

    let cgo = CgatsObject::new();

    // TODO something here

    Ok(cgo)
}