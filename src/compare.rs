use super::*;

pub type CgatsVec = Vec<CgatsObject>;

// Check that all CgatsObjects have the same data type and sample count
pub fn is_comparable(cgats_vec: &CgatsVec) -> bool {
    // If there are less than 2, we can skip out early
    if cgats_vec.len() < 2 { return true; }

    // The first object in the list
    let cgo_prime = &cgats_vec[0];

    for object in cgats_vec[1..].iter() {
        // Make sure they all have the same sample size
        if cgo_prime.len() != object.len() {
            return false;
        }

        // Make sure they all have the same DataFormat
        if cgo_prime.data_format.len() != object.data_format.len() {
            return false;
        }
        for (index, format) in cgo_prime.data_format.iter().enumerate() {
            for object in cgats_vec[1..].iter() {
                if object.data_format[index] != *format {
                    return false
                }
            }
        }
    }

    true
}

fn average(nums: Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for i in &nums {
        sum += i;
    }
    sum / nums.len() as f64
}

pub fn cgats_average(cgats_vec: &CgatsVec) -> CgatsResult<CgatsObject> {
    // The first object in the list
    let cgo_prime = &cgats_vec[0];
    
    // If there's only one or none, we can skip out early
    let vec_count = cgats_vec.len();
    match vec_count {
        1 => return Ok(cgo_prime.clone()),
        0 => return Err(CgatsError::NoData),
        _ => ()
    }

    // Make sure all the objects are comparable
    if !is_comparable(cgats_vec) {
        return Err(CgatsError::CannotCompare);
    } 

    let mut cgo = CgatsObject::new_with_format(
        cgo_prime.data_format.clone()
    );

    let mut cgv: Vec<CgatsMap> = Vec::new();
    for v in cgats_vec {
        cgv.push(v.data_map.clone());
    }

    cgo.data_map = CgatsMap::map_average(cgv)?;

    Ok(cgo)
}