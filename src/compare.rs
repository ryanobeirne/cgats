use super::*;
use std::iter::FromIterator;

pub type CgatsSet = Vec<CgatsObject>;

#[derive(Debug, Clone)]
pub struct CgatsVec {
    pub inner: CgatsSet,
}

impl CgatsVec {
    pub fn new() -> Self {
        Self { inner: CgatsSet::new() }
    }

    pub fn push(&mut self, value: CgatsObject) {
        self.inner.push(value)
    }

    pub fn pop(&mut self) -> Option<CgatsObject> {
        self.inner.pop()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn from_files<T: AsRef<Path>>(files: &Vec<T>) -> Self {
        files.iter()
            .filter_map(|f| CgatsObject::from_file(f).ok())
            .collect()
    }

    // Check that all CgatsObjects have the same data type and sample count
    pub fn is_comparable(&self) -> bool {
        // If there are less than 2, we can skip out early
        if self.len() < 2 { return true; }

        // The first object in the list
        let cgo_prime = &self.inner[0];
        if cgo_prime.is_empty() { return false; }

        self.same_formats() && self.same_sample_count()
    }

    pub fn raw_from_prime(&self, cgats_map: &CgatsMap) -> CgatsResult<RawVec> {
        if self.is_empty() { return Err(CgatsError::CannotCompare) }

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
        raw_vec.push(cgo_prime.data_format.iter().map(|f| f.to_string()).collect());
        raw_vec.push(vec!("END_DATA_FORMAT".to_string()));

        // Push on the DATA
        raw_vec.push(vec!["BEGIN_DATA".to_string()]);
        // This is very important
        let data_vec = cgats_map.to_data_vec();
        for v in data_vec {
            raw_vec.push(v);
        }
        raw_vec.push(vec!["END_DATA".to_string()]);

        Ok(raw_vec)
    }

    pub fn to_map_vec(&self) -> MapVec {
        self.inner.iter()
            .map(|cgo|
                cgo.data_map.clone())
            .collect()
    }

    pub fn average(&self) -> CgatsResult<CgatsObject> {
        // The first object in the list
        let cgo_prime = &self.inner[0];
        
        // If there's only one or none, we can skip out early
        let vec_count = self.len();
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
        let map_vec = self.to_map_vec();

        // Use filler from first object
        let mut cgo = CgatsObject::derive_from(&cgo_prime);
        // Average the DataMaps
        cgo.data_map = map_vec.average()?;
        // Use the first object to fill in the blanks
        cgo.raw_vec = self.raw_from_prime(&cgo.data_map)?;

        // Append sample count to end of first line
        cgo.raw_vec.inner[0].push(format!("Average of {}", vec_count));

        Ok(cgo)
    }

    pub fn same_sample_count(&self) -> bool {
        let cgo_prime = &self.inner[0];

        for object in &self.inner {
            if object.len() != cgo_prime.len() {
                return false;
            }
        }

        true
    }

    pub fn same_formats(&self) -> bool {
        let cgo_prime = &self.inner[0];

        for object in &self.inner {
            // Make sure they all have the same DataFormat
            for format_type in &cgo_prime.data_format {
                if !object.data_format.contains(&format_type) {
                    return false;
                }
            }
            for format_type in &object.data_format {
                if !cgo_prime.data_format.contains(&format_type) {
                    return false;
                }
            }
        }

        true
    }

    pub fn concatenate(&self) -> CgatsResult<CgatsObject> {
        let cgo_prime = &self.inner[0];

        // If there's only 1 or none, we can skip out early
        match self.len() {
            1 => return Ok(cgo_prime.clone()),
            0 => return Err(CgatsError::CannotCompare),
            _ => ()
        }

        // Error if DATA_FORMATS are not the same
        if ! self.same_formats() { return Err(CgatsError::CannotCompare); }

        // Start with the first DATA_FORMAT
        let mut cgo = cgo_prime.clone();

        // Loop through the objects and append the raw_vecs
        for object in &self.inner[1..] {
            cgo.append(&mut object.clone());
        }

        // Create the data_map
        cgo.map()?;

        // Renumber SAMPLE_ID's
        cgo.reindex_sample_id();

        Ok(cgo)
    }
}

impl FromIterator<CgatsObject> for CgatsVec {
    fn from_iter<I: IntoIterator<Item=CgatsObject>>(iter: I) -> Self {
        let mut c = Self::new();

        for i in iter {
            c.push(i);
        }

        c
    }
}

#[derive(Debug, Clone)]
pub struct MapVec {
    inner:  Vec<CgatsMap>,
}

impl MapVec {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn push(&mut self, value: CgatsMap) {
        self.inner.push(value)
    }

    pub fn pop(&mut self) -> Option<CgatsMap> {
        self.inner.pop()
    }

    pub fn average(&self) -> CgatsResult<CgatsMap> {
        let mut cgm = CgatsMap::new();

        for map in &self.inner {
            for ((index, format), value) in &map.inner {
                let key = (*index, *format);
                if format.is_float() {
                    let current = match cgm.inner.get(&key) {
                        Some(c) => c.float,
                        None => 0 as CgatsFloat,
                    };
                    let float = current + &value.float / *&self.len() as CgatsFloat;
                    cgm.inner.insert( key, CgatsValue::from_float(float) );
                } else {
                    if !cgm.inner.contains_key(&key) {
                        cgm.inner.insert(key, value.clone());
                    }
                }
            }
        }

        Ok(cgm)
    }
}

impl FromIterator<CgatsMap> for MapVec {
    fn from_iter<I: IntoIterator<Item=CgatsMap>>(iter: I) -> Self {
        let mut c = Self::new();

        for i in iter {
            c.push(i);
        }

        c
    }
}

pub fn round_to(float: CgatsFloat, places: i32) -> CgatsFloat {
    let mult = (10 as CgatsFloat).powi(places);
    (float * mult).round() / mult
}
