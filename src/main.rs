extern crate cgats;

#[macro_use]
extern crate clap;

use std::fs::File;
// use std::path::Path;
// use std::io::BufReader;
// use std::io::BufRead;
// use std::collections::HashMap;
// use std::io::stdin;

fn main() {
    //Parse command line arguments with clap
    let matches = clap_app!(cgats =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg FILE: +multiple "CGATS File to process")
    ).get_matches();

    let clap_files: Vec<&str> = matches.values_of("FILE").unwrap().collect();

    for clap_file in clap_files {
        let file = File::open(clap_file).unwrap();
        let mut cgd: cgats::CgatsVec = Vec::new();

        cgats::read_cgats_to_vec(&mut cgd, file);

        for line in cgd {
            println!("{:?}", line);
        }
    }
}
