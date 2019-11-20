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
        if let Some(Ok(line)) = lines.next() {
            cgats.vendor = Vendor::from_str(&line)?;
        } else {
            return boxerr!(Error::EmptyFile);
        }

        // Loop through lines of reader
        while let Some(Ok(line)) = lines.next() {
            let trim = line.trim_start_matches(' ').trim_end();
            if trim.is_empty() { continue }

            if trim == "BEGIN_DATA_FORMAT" {
                if let Some(Ok(line)) = lines.next() {
                    for field in line.split('\t') {
                        cgats.fields.push(Field::from_str(field)?);
                    }
                }
                if let Some(Ok(line)) = lines.next() {
                    if line.trim() != "END_DATA_FORMAT" {
                        return boxerr!(Error::NoDataFormat);
                    }
                }
            } else if trim == "BEGIN_DATA" {
                while let Some(Ok(line)) = lines.next() {
                    if line.trim() == "END_DATA" {
                        break;
                    }

                    // Split line on tabs.
                    // Error if the data length doesn't match the number of fields.
                    let sample = line.split('\t').collect::<Sample>();
                    if sample.n_values() != cgats.fields.len() {
                        return boxerr!(Error::FormatDataMismatch);
                    }

                    cgats.samples.push(sample);
                }
            } else {
                cgats.metadata.push(trim.into());
            }
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
    let cgats0 = Cgats::from_file("test_files/cgats0.txt");
    let cgats1 = Cgats::from_file("test_files/empty");
    dbg!(&cgats0, &cgats1);

    assert!(cgats0.is_ok());
    assert!(cgats1.is_err());
    Ok(())
}
