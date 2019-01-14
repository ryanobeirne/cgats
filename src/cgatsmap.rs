use super::*;
use std::fmt::Write;

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

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn from_raw_vec(raw_vec: &RawVec) -> CgatsResult<Self> {
        let mut inner: DataMap = BTreeMap::new();
        
        let data_format = raw_vec.extract_data_format()?;
        let data = raw_vec.extract_data()?;

        for (line_index, line) in data.inner.iter().enumerate() {
            for (index, format) in data_format.iter().enumerate() {
                if *format != DataFormatType::BLANK {
                    inner.insert(
                        (line_index, *format),
                        CgatsValue::from_string(&line[index])
                    );
                }
            }
        }

        Ok(Self {inner})
    }

    pub fn to_data_vec(&self) -> DataVec {
        let mut data_vec = DataVec::new();
        let mut index_map: BTreeMap<usize, bool> = BTreeMap::new();
        let mut format_map: BTreeMap<DataFormatType, bool> = BTreeMap::new();

        for (index, format) in self.inner.keys() {
            index_map.insert(*index, true);
            format_map.insert(*format, true);
        }

        for index in index_map.keys() {
            let mut data_line = DataLine::new();
            for format in format_map.keys() {
                data_line.push(
                    self.inner.get(
                        &(*index, *format)
                    ).unwrap()
                    .clone()
                    .value
                );
            }
            data_vec.insert(*index, data_line);
        }

        data_vec
    }

    pub fn to_cgats_object(&self) -> CgatsObject {
        let mut cgo = CgatsObject::new_with_type(CgatsType::Cgats);

        cgo.raw_vec = RawVec { inner: vec![vec!["CGATS.17".to_string()]] };
        cgo.data_format = self.data_format();
        cgo.data_map.inner = self.inner.clone();

        cgo
    }

    pub fn from_file<T: AsRef<Path>>(file: T) -> CgatsResult<Self> {
        let mut raw_vec = RawVec::new();
        raw_vec.read_file(file)?;

        Self::from_raw_vec(&raw_vec)
    }

    pub fn sample_count(&self) -> usize {
        match self.inner.iter()
            .map(|((index, _), _)| index)
            .max() {
                Some(u) => *u + 1,
                None => 0
            }
    }

    // Extract DATA_FORMAT from CgatsMap
    pub fn data_format(&self) -> DataFormat {
        self.inner.keys()
            .filter(|(index, _)| *index == 0)
            .map(|(_, format)| *format)
            .collect()
    }

    pub fn append_column(&mut self, other: &mut Self) -> CgatsResult<()> {
        if other.sample_count() != self.sample_count() {
            return Err(CgatsError::CannotCompare)
        }

        self.inner.append(&mut other.inner);

        Ok(())
    }

    pub fn sample_id_map(&self) -> Self {
        CgatsMap {
            inner: self.inner
                .keys()
                .map(|(index, _)|
                    ( (*index, DataFormatType::SAMPLE_ID), CgatsValue::from_string(&index.to_string()) )
                ).collect()
        }
    }

    // Rename/renumber SAMPLE_ID's to match index
    pub fn reindex_sample_id(&mut self) {
        for (index,(_, value)) in self.inner.iter_mut()
            .filter(|((_,k), _)| k == &DataFormatType::SAMPLE_ID)
            .enumerate()
        {
            *value = CgatsValue::from_float((index + 1) as CgatsFloat);
        }
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
