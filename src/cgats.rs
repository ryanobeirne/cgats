use super::*;

use std::path::Path;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Cgats {
    pub vendor: Option<Vendor>,
    pub meta: DataVec,
    pub fields: DataFormat,
    pub data_map: DataMap,
}

impl Cgats {
    pub fn new() -> Cgats {
        Cgats::default()
    }

    pub fn sample_count(&self) -> usize {
        self.data_map.len()
    }

    pub fn new_with_vendor(vendor: Vendor) -> Cgats {
        Cgats {
            vendor: Some(vendor),
            ..Cgats::default()
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> CgatsResult<Cgats> {
        let raw = DataVec::from_file(path)?;

        let vendor = raw.get_vendor();
        let meta = raw.extract_meta_data();
        let fields = raw.extract_data_format()?;
        let data_map = raw.to_data_map()?;

        Ok(Cgats {
            vendor,
            meta,
            fields,
            data_map,
        })
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, file: P) -> CgatsResult<()> {
        let f = File::create(file)?;
        let mut buf = BufWriter::new(f);
        write!(buf, "{}", self.write())?;
        Ok(())
    }

    pub fn write(&self) -> String {
        let mut string = String::new();

        string.push_str(&self.write_meta());
        string.push_str(&self.write_fields());
        string.push_str(&self.write_data_map());

        string
    }

    fn write_meta(&self) -> String {
        format!("{}", self.meta)
    }

    fn write_fields(&self) -> String {
        let mut s = String::new();

        if self.vendor == Some(Vendor::ColorBurst) {
            return s;
        }

        s.push_str("BEGIN_DATA_FORMAT\n");

        for field in &self.fields {
            s.push_str(&format!("{}\t", field));
        }

        s.pop();
        s.push_str("\nEND_DATA_FORMAT\n");

        format!("{}", s)
    }

    fn write_data_map(&self) -> String {
        let mut string = "BEGIN_DATA\n".to_string();

        string.push_str(
            &self.data_map.iter()
                .map(|(_index, sample)| format!("{}\n", sample))
                .collect::<String>()
        );

        string.push_str("END_DATA\n");

        string
    }

}

impl Default for Cgats {
    fn default() -> Cgats {
        Cgats {
            vendor: None,
            fields: DataFormat::new(),
            data_map: DataMap::new(),
            meta: DataVec::new(),
        }
    }
}

impl fmt::Display for Cgats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vendor = match &self.vendor {
            Some(v) => v.to_string(),
            None => "None".to_string(),
        };

        let format = format!("{}({}):{:?}", vendor, &self.sample_count(), self.fields);

        write!(f, "{}", format)
    }
}

#[test]
fn cgats_display() -> CgatsResult<()> {
    let cgats = Cgats::from_file("test_files/cgats1.tsv")?;
    let colorburst = Cgats::from_file("test_files/colorburst0.txt")?;
    let curve = Cgats::from_file("test_files/curve0.txt")?;

    assert_eq!(cgats.to_string(),       "Cgats(11):[SAMPLE_ID, SAMPLE_NAME, CMYK_C, CMYK_M, CMYK_Y, CMYK_K]");
    assert_eq!(colorburst.to_string(),  "ColorBurst(84):[D_RED, D_GREEN, D_BLUE, D_VIS, LAB_L, LAB_A, LAB_B]");
    assert_eq!(curve.to_string(),       "Curve(21):[SAMPLE_ID, SAMPLE_NAME, CMYK_C, CMYK_M, CMYK_Y, CMYK_K]");

    Ok(())
}

#[test]
fn write_meta() -> CgatsResult<()> {
    let cgats = Cgats::from_file("test_files/cgats1.tsv")?;
    assert_eq!(
        cgats.write_meta(),
        "CGATS.17\n"
    );

    let colorburst = Cgats::from_file("test_files/colorburst0.txt")?;
    assert_eq!(
        colorburst.write_meta(),
        "ColorBurst\n"
    );

    Ok(())
}

#[test]
fn write_fields() -> CgatsResult<()> {
    let cgats = Cgats::from_file("test_files/cgats1.tsv")?;
    assert_eq!(
        cgats.write_fields(),
        "BEGIN_DATA_FORMAT\nSAMPLE_ID\tSAMPLE_NAME\tCMYK_C\tCMYK_M\tCMYK_Y\tCMYK_K\nEND_DATA_FORMAT\n"
    );

    let curve = Cgats::from_file("test_files/curve0.txt")?;
    assert_eq!(
        curve.write_fields(),
        "BEGIN_DATA_FORMAT\nSAMPLE_ID\tSAMPLE_NAME\tCMYK_C\tCMYK_M\tCMYK_Y\tCMYK_K\nEND_DATA_FORMAT\n"  
    );

    let colorburst = Cgats::from_file("test_files/colorburst0.txt")?;
    assert_eq!(colorburst.write_fields(), "");

    Ok(())
}

#[test]
fn write_data_map() -> CgatsResult<()> {
    let cgats = Cgats::from_file("test_files/cgats1.tsv")?;
    println!("{}", cgats.write_data_map());
    Ok(())
}