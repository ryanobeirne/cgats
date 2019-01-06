#[macro_use]
extern crate clap;

extern crate cgats;
use cgats::*;

mod cli;
mod config;
use config::Config;

fn main() -> CgatsResult<()> {
    //Parse command line arguments with clap
    let matches = cli::build_cli().get_matches();
    let config = Config::build(&matches);
    // println!("{}", config);

    if config.files.is_empty() {
        eprintln!("{}", matches.usage()); 
        std::process::exit(1);
    }

    match &config.command {
        Some(cmd) => {
            let cgo = config.execute()?;
            match matches.subcommand_matches(cmd.display())
                .expect(
                    &format!(
                        "Did not find subcommand_matches for '{}'",
                        cmd.display()
                ))
                .value_of("output")
            {
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