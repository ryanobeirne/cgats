use std::fs::File;
// use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

pub type CgatsVec = Vec<Vec<String>>;

pub enum CgatsType {
    Cgats,
    ColorBurst,
    Curve,
    Unknown,
}

pub struct CgatsData {
    pub data: CgatsVec,
    pub data_type: Option<CgatsType>,
}

impl CgatsData {
    pub fn new(data: CgatsVec) -> Self {
        Self {
            data,
            data_type: None,
        }
    }
}

pub fn read_cgats_to_vec(cgd: &mut Vec<Vec<String>>, file: File) -> () {
    for line in BufReader::new(file).lines() {
        let text = match line {
            Ok(text) => String::from(text.trim_right()),
            Err(_) => String::from("")
        };

        let cr_split = text.split("\r");

        let mut v_cr:Vec<String> = Vec::new();

        for cr_line in cr_split {
            if ! cr_line.is_empty() {
                v_cr.push(String::from(cr_line.trim_right()));
            }
        }

        for split_line in v_cr {
            let split = split_line.split("\t");
            let mut v: Vec<String> = Vec::new();

            for item in split {
                v.push(String::from(item.trim()));
            }

            cgd.push(v);
        }
    } 
}

