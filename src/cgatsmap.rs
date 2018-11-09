use super::*;
use std::fmt::Write;
use std::iter::FromIterator;

// BTreeMap of CGATS Data
pub type MapKey = (usize, DataFormatType);
pub type DataMap = BTreeMap<MapKey, CgatsValue>;

#[derive(Debug, PartialEq, Clone)]
pub struct CgatsMap {
    pub inner: DataMap
}

impl CgatsMap {
    pub fn new() -> Self {
        Self { inner: DataMap::new() }
    }

    pub fn from_raw_vec(raw_vec: &RawVec) -> CgatsResult<Self> {
        let mut inner: DataMap = BTreeMap::new();
        
        let data_format = raw_vec.extract_data_format()?;
        let data = raw_vec.extract_data()?;

        for (line_index, line) in data.inner.iter().enumerate() {
            for (index, format) in data_format.iter().enumerate() {
                inner.insert(
                    (line_index, *format),
                    CgatsValue::from_string(&line[index])
                );
            }
        }

        Ok(Self {inner})
    }

    pub fn to_data_vec(&self) -> CgatsResult<DataVec> {
        let mut data_vec = DataVec::new();
        let mut index_map: BTreeMap<usize, bool> = BTreeMap::new();
        let mut format_map: BTreeMap<DataFormatType, bool> = BTreeMap::new();

        for (index, format) in self.inner.keys() {
            index_map.insert(*index, true);
            format_map.insert(*format, true);
        }

        for index in index_map.keys() {
            let mut mini_vec: Vec<String> = Vec::new();
            for format in format_map.keys() {
                mini_vec.push(
                    self.inner.get(
                        &(*index, *format)
                    ).unwrap()
                    .clone()
                    .value
                );
            }
            data_vec.insert(*index, mini_vec);
        }
        
        Ok(data_vec)
    }

    pub fn from_file<T: AsRef<Path>>(file: T) -> CgatsResult<Self> {
        let mut raw_vec = RawVec::new();
        raw_vec.read_file(file)?;

        Self::from_raw_vec(&raw_vec)
    }

}

impl fmt::Display for CgatsMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        write!(buf, "{}", "CgatsMap{")?;

        for (key, val) in &self.inner {
            write!(buf, "{}[{}: {}], ",
                &key.0,
                &key.1,
                val.value
            )?;
        }
        
        buf.pop(); buf.pop();
        write!(buf, "{}", '}')?;

        write!(f, "{}", buf)
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

    pub fn average(&self) -> CgatsResult<CgatsMap> {
        let mut cgm = CgatsMap::new();

        for map in &self.inner {
            for ((index, format), value) in &map.inner {
                let key = (*index, *format);
                if format.is_float() {
                    let current = match cgm.inner.get(&key) {
                        Some(c) => c.float,
                        None => 0_f64
                    };
                    let float = current + &value.float / *&self.len() as f64;
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
