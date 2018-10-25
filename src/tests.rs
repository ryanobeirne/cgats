use super::*;

#[allow(dead_code)]
fn test_files<'a>() -> Vec<&'a str> {
    vec![
        "test_files/cgats_format.tsv",
        "test_files/cgats0.txt",
        "test_files/cgats1.tsv",
        "test_files/colorburst0.txt",
        "test_files/colorburst1.lin",
        "test_files/curve0.txt",
        "test_files/empty",
        "test_files/other",
    ]
}

#[test]
fn data_column() {
    let dc0 = DataColumn {
        header: "SAMPLE_NAME",
        data: vec![
            "Cyan", "Magenta", "Yellow", "Black", "Blue", "Red", "Green", "3cBlack", "4cBlack", "3cGray", "1cGray",
        ]
 };

    let dc1 = DataColumn {
        header: "CMYK_C",
        data: vec![
            100, 0, 0, 0, 100, 0, 100, 100, 100, 50, 0,
        ]
 };

    let dcs = DataSet {
        columns: vec![&dc0, &dc0]
    };

    println!("{:?}", dc0);
    println!("{:?}", dc1);
    println!("--\n{:?}\n--", dcs);
}

#[test]
fn data_format() -> CgatsResult<()> {
    let mut cgv: RawVec<> = Vec::new();
    read_file_to_raw_vec(&mut cgv, "test_files/cgats0.txt")?;

    println!("{:?}", cgv);

    let df = DataFormatType::from("CMYK_C")?;
    assert_eq!(df.display(), "CMYK_C");

    Ok(())
}

#[test]
fn test_extract_data_format() -> CgatsResult<()> {
    let cgo = CgatsObject::from_file("test_files/cgats1.tsv")?;
    let data = extract_data_format(&cgo.raw_data)?;
    println!("{:?}", data);

    Ok(())
}

#[test]
fn test_extract_data() -> CgatsResult<()>{
    let cgo = CgatsObject::from_file("test_files/cgats1.tsv")?;
    let data = extract_data(&cgo.raw_data)?;
    println!("{:?}", data);

    Ok(())
}

#[test]
fn test_extract_data_and_format() -> CgatsResult<()>{
    let cgo = CgatsObject::from_file("test_files/cgats1.tsv")?;
    let format = extract_data_format(&cgo.raw_data)?;
    let data = extract_data(&cgo.raw_data)?;
    println!("FORMAT [{}]:\n{:?}\n\nDATA [{}]:\n{:?}", format.len(), format, data.len(), data);

    for line in data {
        assert_eq!(line.len(), format.len())
    }

    Ok(())
}

#[test]
fn text() {
    let text = "one two 3 \"4\" 5 six seven";
    let split: Vec<f64> = text.split_whitespace().filter_map(|i| i.parse().ok()).collect();
    println!("{:?}", split);
}