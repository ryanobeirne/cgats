use super::*;

pub struct CgatsVec {
    pub inner: Vec<CgatsObject>,
}

impl CgatsVec {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, value: CgatsObject) {
        self.inner.push(value)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn from_files<T: AsRef<Path>>(file_vec: &Vec<T>) -> CgatsResult<Self> {
        let mut cgats_vec = Self::new();

        for file in file_vec {
            cgats_vec.push(CgatsObject::from_file(file)?);
        }

        Ok(cgats_vec)
    }

    // Check that all CgatsObjects have the same data type and sample count
    pub fn is_comparable(&self) -> bool {
        // If there are less than 2, we can skip out early
        if self.len() < 2 { return true; }

        // The first object in the list
        let cgo_prime = &self.inner[0];

        for object in self.inner[1..].iter() {
            // Make sure they all have the same sample size
            if cgo_prime.len() != object.len() {
                return false;
            }

            // Make sure they all have the same DataFormat
            if cgo_prime.data_format.len() != object.data_format.len() {
                return false;
            }
            for (index, format) in cgo_prime.data_format.iter().enumerate() {
                for object in self.inner[1..].iter() {
                    if object.data_format[index] != *format {
                        return false
                    }
                }
            }
        }

        true
    }

    // #[allow(dead_code)]
    // fn average(nums: Vec<f64>) -> f64 {
    //     let mut sum: f64 = 0.0;
    //     for i in &nums {
    //         sum += i;
    //     }
    //     sum / nums.len() as f64
    // }
    pub fn raw_from_prime(&self, cgats_map: &CgatsMap) -> CgatsResult<RawVec> {
        let mut raw_vec = RawVec::new();

        // The first object in the list
        let cgo_prime = &self.inner[0];

        // Collect metadata from first object
        if let Some(metadata) = cgo_prime.metadata() {
            for line in metadata.inner {
                raw_vec.push(line);
            }
        }

        // Push on the DATA_FORMAT
        raw_vec.push(vec!("BEGIN_DATA_FORMAT".to_string()));
        raw_vec.push(cgo_prime.data_format.iter().map(|f| f.display()).collect());
        raw_vec.push(vec!("END_DATA_FORMAT".to_string()));

        // Push on the DATA
        raw_vec.push(vec!["BEGIN_DATA".to_string()]);
        // This is very important
        let data_vec = cgats_map.to_data_vec()?;
        for v in data_vec {
            raw_vec.push(v);
        }
        raw_vec.push(vec!["END_DATA".to_string()]);

        Ok(raw_vec)
    }

    pub fn average(&self) -> CgatsResult<CgatsObject> {
        // The first object in the list
        let cgo_prime = &self.inner[0];
        
        // If there's only one or none, we can skip out early
        let vec_count = self.inner.len();
        match vec_count {
            1 => return Ok(cgo_prime.clone()),
            0 => return Err(CgatsError::NoData),
            _ => ()
        }

        // Make sure all the objects are comparable
        if !self.is_comparable() {
            return Err(CgatsError::CannotCompare);
        } 

        // Collect all the DataMaps into a MapVec
        let map_vec: MapVec = self.inner.iter().map(|cgvo|
            cgvo.data_map.clone()
        ).collect();

        // Use filler from first object
        let mut cgo = CgatsObject::derive_from(&cgo_prime);
        // Average the DataMaps
        cgo.data_map = map_vec.average()?;
        // Use the first object to fill in the blanks
        cgo.raw_vec = self.raw_from_prime(&cgo.data_map)?;

        Ok(cgo)
    }
}