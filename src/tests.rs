use super::*;

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
        column: vec![&dc0, &dc0]
    };

    println!("{:?}", dc0);
    println!("{:?}", dc1);
    println!("--\n{:?}\n--", dcs);
}

#[test]
fn data_format() {
    let mut cgv: CgatsVec = Vec::new();
    read_file_to_cgats_vec(&mut cgv, "test_files/cgats0.txt");

    println!("{:?}", cgv);

    let df = DataFormatType::from("CMYK_C").unwrap();
    assert_eq!(df.display(), "CMYKC");
}