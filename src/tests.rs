use super::*;

// List of all test files relative to crate root
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

// Test that DataColumns can be Vectors of different types (Vec<&str>, Vec<i32>)
#[test]
fn data_column() {
    let dc0 = DataColumn {
        data_type: DataFormatType::SAMPLE_NAME,
        data: vec![
            "Cyan", "Magenta", "Yellow", "Black", "Blue", "Red", "Green", "3cBlack", "4cBlack", "3cGray", "1cGray",
        ]
 };

    let dc1 = DataColumn {
        data_type: DataFormatType::CMYK_C,
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

// Test the conversion of DataFormatTypes
#[test]
fn data_format() -> CgatsResult<()> {
    let mut cgv: RawVec<> = Vec::new();
    read_file_to_raw_vec(&mut cgv, "test_files/cgats0.txt")?;

    println!("{:?}", cgv);

    let df = DataFormatType::from("CMYK_C")?;
    assert_eq!(df.display(), "CMYK_C");

    Ok(())
}

// Test the extraction of DATA_FORMAT
#[test]
fn test_extract_data_format() -> CgatsResult<()> {
    use format::DataFormatType::*;
    let cgo = CgatsObject::from_file("test_files/cgats1.tsv")?;
    let format = extract_data_format(&cgo.raw_data)?;
    println!("{:?}", format);

    let format_vec = vec![SAMPLE_ID, SAMPLE_NAME, CMYK_C, CMYK_M, CMYK_Y, CMYK_K];

    assert_eq!(format_vec, format);

    Ok(())
}

// Test the extraction of DATA
#[test]
fn test_extract_data() -> CgatsResult<()>{
    let cgo = CgatsObject::from_file("test_files/cgats1.tsv")?;
    let data = extract_data(&cgo.raw_data)?;
    println!("{:?}", data);

    let data_vec = vec![
       vec!["1",  "Cyan",    "100", "0",   "0",   "0"  ],
       vec!["2",  "Magenta", "0",   "100", "0",   "0"  ],
       vec!["3",  "Yellow",  "0",   "0",   "100", "0"  ],
       vec!["4",  "Black",   "0",   "0",   "0",   "100"],
       vec!["5",  "Blue",    "100", "100", "0",   "0"  ],
       vec!["6",  "Red",     "0",   "100", "100", "0"  ],
       vec!["7",  "Green",   "100", "0",   "100", "0"  ],
       vec!["6",  "3cBlack", "100", "100", "100", "0"  ],
       vec!["8",  "4cBlack", "100", "100", "100", "100"],
       vec!["9",  "3cGray",  "50",  "40",  "40",  "0"  ],
       vec!["10", "1cGray",  "0",   "0",   "0",   "50" ],
    ];

    assert_eq!(data_vec, data);

    Ok(())
}

// Test the extraction of DATA and DATA_FORMAT
#[test]
fn test_extract_data_and_format() -> CgatsResult<()>{
    let cgo = CgatsObject::from_file("test_files/cgats0.txt")?;
    let format = extract_data_format(&cgo.raw_data)?;
    let data = extract_data(&cgo.raw_data)?;
    println!("FORMAT [{}]:\n{:?}\n\nDATA [{}]:\n{:?}", format.len(), format, data.len(), data);

    for line in data {
        assert_eq!(line.len(), format.len())
    }

    Ok(())
}

// This test is a reminder to parse a DataColumn
#[test]
fn text() {
    let text = "one two 3 \"4\" 5 six seven";
    let split: Vec<f64> = text.split_whitespace().filter_map(|i| i.parse().ok()).collect();
    println!("{:?}", split);
}

// Test the parsing of CgatsType from first line of file
// cargo test --lib -- --nocapture cgats_type
#[test]
fn cgats_type() -> CgatsResult<()>{
    for file in test_files() {
        let cgo = CgatsObject::from_file(file);
        match cgo {
            Ok(object) => println!("{}: {:?}", file, object.cgats_type),
            Err(e) => eprintln!("{}: {}", file, e),
        }
    }

    Ok(())
}