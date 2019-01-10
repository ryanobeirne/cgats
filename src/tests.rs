use super::*;

// List of all test files relative to crate root
fn test_files() -> Vec<&'static str> {
    vec![
        "test_files/cgats_format.tsv",
        "test_files/cgats0.txt",
        "test_files/cgats1.tsv",
        "test_files/cgats2.tsv",
        "test_files/cgats3.tsv",
        "test_files/cgats4.tsv",
        "test_files/colorburst0.txt",
        "test_files/colorburst1.lin",
        "test_files/curve0.txt",
        "test_files/empty",
        "test_files/other",
    ]
}


// Test the conversion of DataFormatTypes
#[test]
fn data_format() -> CgatsResult<()> {
    let cgv = RawVec::from_file("test_files/cgats0.txt")?;

    println!("{:?}", cgv);

    let df = DataFormatType::from_str("CMYK_C")?;
    assert_eq!(df.to_string(), "CMYK_C");

    Ok(())
}

// Test the extraction of DATA_FORMAT
#[test]
fn test_extract_data_format() -> CgatsResult<()> {
    use format::DataFormatType::*;
    let cgo = CgatsObject::from_file("test_files/cgats1.tsv")?;
    let format = cgo.raw_vec.extract_data_format()?;
    println!("{:?}", format);

    let format_vec = vec![SAMPLE_ID, SAMPLE_NAME, CMYK_C, CMYK_M, CMYK_Y, CMYK_K];

    assert_eq!(format_vec, format);

    Ok(())
}

// Test the extraction of DATA
#[test]
fn test_extract_data() -> CgatsResult<()>{
    let data = CgatsObject::from_file("test_files/cgats1.tsv")?.data()?;
    println!("{:?}", data);

    let data_vec = vec![
       vec!["1",  "Cyan",    "100", "0",   "0",   "0"  ],
       vec!["2",  "Magenta", "0",   "100", "0",   "0"  ],
       vec!["3",  "Yellow",  "0",   "0",   "100", "0"  ],
       vec!["4",  "Black",   "0",   "0",   "0",   "100"],
       vec!["5",  "Blue",    "100", "100", "0",   "0"  ],
       vec!["6",  "Red",     "0",   "100", "100", "0"  ],
       vec!["7",  "Green",   "100", "0",   "100", "0"  ],
       vec!["8",  "3cBlack", "100", "100", "100", "0"  ],
       vec!["9",  "4cBlack", "100", "100", "100", "100"],
       vec!["10", "3cGray",  "50",  "40",  "40",  "0"  ],
       vec!["11", "1cGray",  "0",   "0",   "0",   "50" ],
    ];

    assert_eq!(data_vec, data.inner);

    Ok(())
}

// Test the extraction of DATA and DATA_FORMAT
#[test]
fn test_extract_data_and_format() -> CgatsResult<()>{
    let cgo = CgatsObject::from_file("test_files/cgats0.txt")?;
    let format = cgo.raw_vec.extract_data_format()?;
    let data = cgo.raw_vec.extract_data()?;
    println!("FORMAT [{}]:\n{:?}\n\nDATA [{}]:\n{:?}", format.len(), format, data.len(), data);

    for line in data.inner {
        assert_eq!(line.len(), format.len())
    }

    Ok(())
}

// This test is a reminder to parse a DataColumn
#[test]
fn text() {
    let text = "one two 3 \"4\" 5 six seven";
    let split: Vec<CgatsFloat> = text.split_whitespace().filter_map(|i| i.parse().ok()).collect();
    println!("{:?}", split);
}

// Test the parsing of CgatsType from first line of file
// cargo test --lib -- --nocapture cgats_type
#[test]
fn cgats_type() -> CgatsResult<()>{
    for file in test_files() {
        let cgo = CgatsObject::from_file(file);
        match cgo {
            Ok(object) => {
                println!("{}: {:?}", file, object.cgats_type);
                assert!(object.cgats_type.is_some());
            },
            Err(e) => eprintln!("{}: {}", file, e),
        }
    }

    Ok(())
}

#[test]
fn cgats_map() {
    for file in test_files() {
        let cgo = CgatsObject::from_file(file);
        let mut s = String::new();

        match cgo {
            Ok(cgo) => {
                for ((id, format), value) in cgo.data_map.inner {
                    println!("{}, {}:\t{}", id, format, value);
                }
            },
            Err(e) => s.push_str( &format!("'{}': {}", file, e) )
        }

        if !s.is_empty() { eprintln!("{}", s); }
    }
}

#[test]
fn cgo_print() -> CgatsResult<()> {
    let cgo = CgatsObject::from_file("test_files/cgats1.tsv")?;
    println!("{}", cgo.print()?);

    Ok(())
}

#[test]
fn meta() -> CgatsResult<()> {
    let cgo = CgatsObject::from_file("test_files/curve0.txt")?;
    let meta = cgo.raw_vec.extract_meta_data();

    match meta {
        Some(m) => Ok(println!("{:?}", m)),
        None => Err(CgatsError::NoData)
    }
}

#[test]
fn compare_average() -> CgatsResult<()> {
    let avg = CgatsVec::from_files(
        &vec![
            "test_files/cgats1.tsv",
            "test_files/cgats2.tsv",
        ]
    ).average()?;

    println!("{}", &avg.print()?);

    let expected = CgatsObject::from_file("test_files/cgats5.tsv")?;
    assert_eq!(avg, expected);

    Ok(())
}

#[test]
fn btreemap() -> CgatsResult<()> {
    let cgo = CgatsObject::from_file("test_files/cgats1.tsv")?;
    let cgm = cgo.data_map;
    let val = cgm.inner.get(&(0,DataFormatType::SAMPLE_NAME)).unwrap();
    println!("{}", val);

    Ok(())
}

#[test]
fn column_order() -> CgatsResult<()> {
    let cg0 = CgatsObject::from_file("test_files/cgats1.tsv")?;
    let cgv = CgatsVec::from_files(&vec!["test_files/cgats1.tsv", "test_files/cgats4.tsv"]);
    let avg = cgv.average()?;
    println!("{}", avg);
    assert_eq!(cg0.data_map, avg.data_map);
    Ok(())
}

#[test]
fn reindex() -> CgatsResult<()> {
    let cgv = CgatsVec::from_files(&vec![
        "test_files/cgats1.tsv",
        "test_files/cgats2.tsv",
        "test_files/cgats3.tsv",
        "test_files/cgats4.tsv",
    ]);

    let cat = cgv.concatenate()?;

    println!("{}", cat.print()?);    

    let max_id = cat.data_map.inner.iter()
        .filter(|(k,_)| k.1 == DataFormatType::SAMPLE_ID)
        .map(|(_, v)| v.float as usize)
        .max()
        .expect("No SAMPLE_ID found!");

    assert_eq!(cat.len(), max_id);

    Ok(())
}

#[test]
fn cat_cb() -> CgatsResult<()> {
    let cgv = CgatsVec::from_files(&vec![
        "test_files/colorburst0.txt",
        "test_files/colorburst1.lin",
    ]);

    let cat = cgv.concatenate()?;

    println!("{}", cat.print()?);

    Ok(())
}

#[test]
fn all_eq() -> CgatsResult<()> {
    let cgv = CgatsVec::from_files(&vec![
        "test_files/cgats0.txt",
        "test_files/cgats0.txt",
        "test_files/cgats0.txt",
        "test_files/cgats0.txt",
        "test_files/cgats6.txt",
    ]);

    assert!(cgv.all_eq());

    Ok(())
}