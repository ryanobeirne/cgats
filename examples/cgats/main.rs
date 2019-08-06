mod cli;
mod config;
use config::Config;
use cgats::*;
use std::io::{BufWriter, stdout};
use std::fs::File;

fn main() -> Result<()> {
    //Parse command line arguments with clap
    let matches = cli::build_cli().get_matches();
    let config = Config::build(&matches);

    #[cfg(debug_assertions)]
    dbg!(&config);

    if config.files.is_empty() {
        eprintln!("{}", matches.usage()); 
        std::process::exit(1);
    }

    if let Some(file) = matches.value_of("OUTPUTFILE") {
        config.execute(&mut BufWriter::new(File::create(file)?))?
    } else {
        config.execute(&mut BufWriter::new(stdout()))?
    };

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::fs::read_to_string;

    type Result<T> = std::result::Result<T, Box<std::error::Error>>;

    macro_rules! cmd_eq_file {
        ($a:ident, $b:expr, $c:expr) => {
            #[test]
            fn $a() -> Result<()> {
                example_compare($b, $c)
            }
        };
    }

    #[cfg(test)]
    fn example_compare(cmd_string: &str, exp_file: &str) -> Result<()> {
        let calculated = String::from_utf8(Command::new("bash").arg("-c")
            .arg(format!("cargo run --quiet --example=cgats -- {}", cmd_string))
            .output()?
            .stdout
        )?;

        let expected = read_to_string(exp_file)?;

        assert_eq!(calculated, expected);

        Ok(())
    }

    cmd_eq_file!(avg,   "avg test_files/cgats{1,2}.tsv",        "test_files/cgats5.tsv");
    cmd_eq_file!(cat,   "cat test_files/cgats{1,2}.tsv",        "test_files/cgats7.tsv");
    cmd_eq_file!(delta, "delta test_files/colorburst{2,3}.lin", "test_files/deltae0.txt");
    cmd_eq_file!(dereport, "delta --report --output-file=/dev/null test_files/colorburst{2,3}.lin 2>&1", "test_files/dereport0.txt");
}