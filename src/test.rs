use super::*;
use mktemp::Temp;
use std::path::PathBuf;
use std::fs::File;

pub const GOOD_FILES: [&str; 11] = [
    "test_files/cgats0.txt",
    "test_files/cgats1.tsv",
    "test_files/cgats2.tsv",
    "test_files/cgats3.tsv",
    "test_files/cgats4.tsv",
    "test_files/cgats5.tsv",
    "test_files/cgats6.tsv",
    "test_files/colorburst0.txt",
    "test_files/colorburst1.lin",
    "test_files/colorburst2.lin",
    "test_files/curve0.txt",
];

pub const BAD_FILES: [&str; 4] = [
    "test_files/cgats_format.tsv",
    "test_files/empty",
    "test_files/other",
    "nonexistent.derp"
];

#[cfg(test)]
fn test_files_cgats() -> Vec<Cgats> {
    GOOD_FILES.iter()
        .filter_map(|file| Cgats::from_file(file).ok())
        .collect()
}

#[test]
fn good_files() {
    for file in &GOOD_FILES {
        assert!(Cgats::from_file(file).is_ok());
    }
}

#[test]
fn bad_files() {
    for file in &BAD_FILES {
        assert!(Cgats::from_file(file).is_err())
    }
}

#[test]
fn reconstruct() -> CgatsResult<()> {
    for cgats in test_files_cgats() {
        println!("---CGATS---\n{}---CGATS---\n", cgats.format());

        let temp = mktemp()?;
        cgats.write_to_file(&temp)?;
        let reconstructed = Cgats::from_file(&temp)?;
        println!("---RECONSTRUCTED---\n{}---RECONSTRUCTED---\n", reconstructed.format());

        assert_eq!(cgats.format(), reconstructed.format());
        std::fs::remove_file(temp)?;
    }
    Ok(())
}

pub fn mktemp() -> CgatsResult<PathBuf> {
    let temp = Temp::new_file()?.to_path_buf();
    File::create(&temp)?;
    Ok(temp)
}