use crate::*;

use std::fs::File;
use std::convert::TryFrom;
use std::io::{Read, BufReader, BufRead};
use std::str::FromStr;

impl<R: Read> TryFrom<BufReader<R>> for Cgats {
    type Error = BoxError;
    fn try_from(reader: BufReader<R>) -> Result<Self> {
        let mut cgats = Cgats::default();

        // Iterator over lines of the reader
        let mut lines = reader.lines().peekable();

        // Read the first line to find Vendor type
        if let Some(line) = lines.next() {
            cgats.vendor = Vendor::from_str(&line?)?;
        } else {
            return boxerr!(Error::EmptyFile);
        }

        Ok(cgats)
    }
}

impl TryFrom<File> for Cgats {
    type Error = BoxError;
    fn try_from(file: File) -> Result<Self> {
        let cgats = Cgats::try_from(BufReader::new(file))?;
        Ok(cgats)
    }
}

#[test]
fn read_file() -> Result<()> {
    let cgats0 = Cgats::try_from(File::open("test_files/cgats1.tsv")?);
    let cgats1 = Cgats::try_from(File::open("test_files/empty")?);
    dbg!(&cgats0, &cgats1);

    assert!(cgats0.is_ok());
    assert!(cgats1.is_err());
    Ok(())
}
