extern crate cgats;

#[macro_use]
extern crate clap;

// use std::fs::File;
use std::path::Path;
// use std::io::BufReader;
// use std::io::BufRead;
// use std::collections::HashMap;
// use std::io::stdin;

enum Command {
    Average,
    Convert,
}

impl Command {
    fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "average" | "avg" => Some(Command::Average),
            "convert"         => Some(Command::Convert),
            _ => None
        }
    }
}

struct Config {
    command: Option<Command>,
    files: Vec<String>,
}

impl<'a> Config {
    fn collect(matches: clap::ArgMatches<'a>) -> Self {
        let command = Command::from_string(matches.subcommand_name().unwrap());
        let files: Vec<String> = matches.values_of("average").unwrap().map(|s| s.to_string()).collect();
        Self { command, files }
    }
}

fn main() -> cgats::error::CgatsResult<()> {
    //Parse command line arguments with clap
    let matches = clap_app!(cgats =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg FILE: +multiple +required "CGATS File to process.")
        (@subcommand average =>
            (about: "Average values from multiple CGATS files.")
            (@arg FILE: +multiple +required "CGATS File to average.")
        )
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
                // println!("{}", &object);
                println!("{}", &object.print()?);
                // println!("{:?}", &object.format);
                // println!("{:?}", &object.data);
                // println!("{:?}", object.data_map);
                // for ( (index, format), value) in object.data_map.0 {
                //     println!("{}, {}:\t{}", index, format, value);
                // }
            },
            Err(e) => eprintln!("{}: {}", clap_file, e),
        }
    }

    Ok(())
}