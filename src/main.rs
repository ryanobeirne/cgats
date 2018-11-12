extern crate cgats;
use cgats::*;

#[macro_use]
extern crate clap;
use clap::{ArgMatches, Arg, App, SubCommand};

mod config;
use config::Config;

fn main() -> CgatsResult<()> {
    //Parse command line arguments with clap
    let matches = App::new("cgats")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("files")
            .help("CGATS files")
            .value_name("FILE")
            .multiple(true))
        .subcommand(SubCommand::with_name("average")
            .about("Average 2 or more CGATS color files")
            .arg(Arg::with_name("output")
                .value_name("output_file")
                .takes_value(true)
                .short("f")
                .long("file")
                .help("Output results to file"))
            .arg(Arg::with_name("comparefiles")
                .value_name("FILE")
                .multiple(true)
                .required(true))
            )
        .get_matches();

    let config = Config::build(&matches);

    if config.files.len() < 1 {
        eprintln!("{}", matches.usage());
        std::process::exit(1);
    }

    match &config.command {
        Some(cmd) => {
            let cgo = config.execute()?;
            match matches.subcommand_matches(cmd.display()).unwrap().value_of("output") {
                Some(f) => cgo.write_cgats(f)?,
                None => println!("{}", cgo.print()?)
            }
        },
        None => {
            for file in &config.files {
                match CgatsObject::from_file(&file) {
                    Ok(cgo) => println!("'{}':\n\t{}", file, cgo),
                    Err(e) => eprintln!("'{}':\n\t{}", file, e)
                }
            }
        }
    }

    Ok(())
}