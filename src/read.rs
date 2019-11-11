use crate::*;

use std::fs::File;
use std::convert::TryFrom;
use std::io::{Read, BufReader, BufRead};
use std::str::FromStr;

impl<R: Read> TryFrom<BufReader<R>> for Cgats {
    type Error = Error;
    fn try_from(reader: BufReader<R>) -> Result<Self> {
        let mut cgats = Cgats::default();

        let mut lines = reader.lines().peekable();

        if let Some(line) = lines.next() {
            cgats.vendor = Vendor::from_str(&line.expect("ReadLine")).expect("FromStr");
        } else {
            return Err(Error::EmptyFile);
        }

        Ok(cgats)
    }
}

impl TryFrom<File> for Cgats {
    type Error = Error;
    fn try_from(file: File) -> Result<Self> {
        let cgats = Cgats::try_from(BufReader::new(file))?;
        Ok(cgats)
    }
}

#[test]
fn read_file() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let cgats = Cgats::try_from(File::open("test_files/cgats1.tsv")?)?;
    dbg!(cgats);
    Ok(())
}
