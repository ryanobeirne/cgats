use cgats::*;
use clap::ArgMatches;
use std::str::FromStr;
use std::io::{self, Write, stderr, BufWriter};
use deltae::DEMethod;
use crate::DeReport;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Command {
    Display, // Default
    Average,
    Cat,
    Delta,
    // Merge
    // Convert,
}

impl Default for Command {
    fn default() -> Self {
        Command::Display
    }
}

impl Command {
    pub fn from_string(s: &str) -> Self {
        match Command::from_str(s) {
            Ok(cmd) => cmd,
            Err(_) => Command::default()
        }
    }
}

impl FromStr for Command {
    type Err = io::Error;
    fn from_str(s: &str) -> std::result::Result<Command, Self::Err> {
        match s.to_lowercase().as_str() {
            "average" | "avg" => Ok(Command::Average),
            "concatenate" | "cat" | "append" => Ok(Command::Cat),
            "delta" | "deltae" | "de" => Ok(Command::Delta),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Debug)]
pub struct Config{
    pub command: Command,
    pub de_method: DEMethod,
    pub de_report: bool,
    pub files: Vec<String>,
}

impl Config {
    pub fn build(matches: &ArgMatches) -> Self {
        let cmd_name = matches.subcommand_name();
        let subcommand = cmd_name.unwrap_or_default();
        let submatches = matches.subcommand_matches(subcommand);
        let command = Command::from_string(subcommand);

        let de_method = if let Some(de) = matches.value_of("DEMETHOD") {
            DEMethod::from_str(de).unwrap_or_default()
        } else {
            DEMethod::default()
        };

        let de_report = matches.is_present("DEREPORT");

        let files = match cmd_name {
            Some(_cmd) => submatches.expect("SUBCOMMAND")
                .values_of("FILES").unwrap_or_default()
                .map(String::from)
                .collect::<Vec<_>>(),
            None => matches.values_of("FILES").unwrap_or_default()
                .map(String::from)
                .collect::<Vec<_>>(),
        };

        Self { command, de_method, de_report, files}
    }

    pub fn execute<W: Write>(&self, output: &mut BufWriter<W>) -> Result<()> {
        let cgv = CgatsVec::from_files(&self.files);

        match self.command {
            Command::Display => {
                for cgo in cgv.collection.iter() {
                    writeln!(output, "{:?}", cgo)?;
                }
            },

            Command::Average => {
                write!(output, "{}", cgv.average()?)?;
                
            },

            Command::Delta => {
                let cgd = cgv.deltae(self.de_method)?;
                write!(output, "{}", &cgd)?;
                if self.de_report {
                    write!(stderr(), "{}", DeReport::new(&cgd)?)?;
                }
            },

            Command::Cat => {
                write!(output, "{}", cgv.concatenate()?)?;
            }
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            command: Command::default(),
            de_method: DEMethod::default(),
            de_report: false,
            files: Vec::new(),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("Config {\n\tcommand: ");

        s.push_str(&format!("{:?}\n", self.command));

        for file in &self.files {
            s.push_str(&format!("\tfile: {}\n", file));
        }

        s.push('}');

        write!(f, "{}", s)
    }
}