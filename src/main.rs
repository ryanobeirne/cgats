extern crate cgats;

#[macro_use]
extern crate clap;

// use std::fs::File;
use std::path::Path;
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
        (@arg FILE: +multiple +required "CGATS File to process")
    ).get_matches();

    let clap_files: Vec<&str> = matches.values_of("FILE").unwrap().collect();

    let mut err_count = 0;

    for clap_file in clap_files {
        if !Path::new(clap_file).is_file() {
            eprintln!("File does not exist: '{}'", clap_file);
            err_count += 1;
            continue;
        }

        let mut cgd: cgats::CgatsVec = Vec::new();
        cgats::read_file_to_cgats_vec(&mut cgd, clap_file);

        let mut set = cgats::CgatsObject::new(&cgd);

        for line in cgd {
            println!("{:?}", line);
        }
    }

    std::process::exit(err_count);
}
