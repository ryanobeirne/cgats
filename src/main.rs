extern crate cgats;

#[macro_use]
extern crate clap;

// use std::fs::File;
use std::path::Path;
// use std::io::BufReader;
// use std::io::BufRead;
// use std::collections::HashMap;
// use std::io::stdin;

fn main() -> cgats::error::CgatsResult<()> {
    //Parse command line arguments with clap
    let matches = clap_app!(cgats =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg FILE: +multiple +required "CGATS File to process")
    ).get_matches();

    let clap_files: Vec<&str> = matches.values_of("FILE").unwrap().collect();

    for clap_file in clap_files {
        if !Path::new(clap_file).is_file() {
            eprintln!("File does not exist: '{}'", clap_file);
            continue;
        }

        let set = cgats::CgatsObject::from_file(clap_file);

        match set {
            Ok(object) => {
                println!("{}", &object);
                println!("{}", &object.print());
                // println!("{:?}", &object.format);
                // println!("{:?}", &object.data);
                // println!("{:?}", object.data_map);
                for ( (index, format), value) in object.data_map.0 {
                    println!("{}, {}:\t{}", index, format, value);
                }
            },
            Err(e) => eprintln!("{}: {}", clap_file, e),
        }
    }

    Ok(())
}