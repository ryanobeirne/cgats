mod cli;
mod config;
use config::Config;
use cgats::*;

fn main() -> Result<()> {
    //Parse command line arguments with clap
    let matches = cli::build_cli().get_matches();
    let mut config = Config::build(&matches)?;

    if config.files.is_empty() {
        eprintln!("{}", matches.usage()); 
        std::process::exit(1);
    }

    config.execute()?;

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

        assert_eq!(calculated.trim(), expected.trim());

        Ok(())
    }

    cmd_eq_file!(avg,   "avg test_files/cgats{1,2}.tsv",        "test_files/cgats5.tsv");
    cmd_eq_file!(cat,   "cat test_files/cgats{1,2}.tsv",        "test_files/cgats7.tsv");
    cmd_eq_file!(delta, "delta test_files/colorburst{2,3}.lin", "test_files/deltae0.txt");
    cmd_eq_file!(dereport_2000, "delta -rf/dev/null test_files/colorburst{2,3}.lin 2>&1", "test_files/dereport0.txt");
    cmd_eq_file!(dereport_1976, "delta --method=1976 -rf/dev/null test_files/colorburst{2,3}.lin 2>&1", "test_files/dereport1.txt");
}