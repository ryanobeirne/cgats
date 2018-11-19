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
        
        Ok(data_vec)
    }

    pub fn from_file<T: AsRef<Path>>(file: T) -> CgatsResult<Self> {
        let mut raw_vec = RawVec::new();
        raw_vec.read_file(file)?;

        Self::from_raw_vec(&raw_vec)
    }

    pub fn max_index(&self) -> usize {
        let mut max_index = 0;

        for ((index, _), _) in &self.inner {
            if index > &max_index {
                max_index = *index
            }
        }

        max_index
    }

    // Extract DATA_FORMAT from CgatsMap
    pub fn data_format(&self) -> CgatsResult<DataFormat> {
        let mut data_format = DataFormat::new();

        // Loop through map keys and push the format
        for (_, format) in self.inner.keys() {
            if !data_format.contains(format) {
                data_format.push(*format);
            }
        }

        // Error if empty format
        if data_format.len() < 1 {
            return Err(CgatsError::UnknownFormatType)
        }

        // Check that the first format type is SAMPLE_ID unless it's ColorBurst
        if data_format[0] != DataFormatType::SAMPLE_ID &&
            data_format != format::ColorBurstFormat() {
                return Err(CgatsError::InvalidID);
        }

        Ok(data_format)
    }

    // Rename/renumber SAMPLE_ID's to match index
    pub fn reindex_sample_id(&mut self) {
        for (key, value) in self.inner.iter_mut() {
            if key.1 == DataFormatType::SAMPLE_ID {
                *value = CgatsValue::from_string( &(key.0).to_string() );
            }
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
