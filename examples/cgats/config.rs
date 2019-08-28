use cgats::*;
use clap::ArgMatches;
use std::str::FromStr;
use std::io::{self, Write, stderr, stdout, Stdout, BufWriter};
use deltae::DEMethod;
use crate::DeReport;
use std::fs::File;
use std::path::Path;
use super::cli;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Command {
    Display, // Default
    Print,
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
            "display" => Ok(Command::Display),
            "print" => Ok(Command::Print),
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
enum CgatsWriter {
    File(BufWriter<File>),
    Stdout(BufWriter<Stdout>),
}

impl CgatsWriter {
    fn file<P: AsRef<Path>>(file: P) -> Result<Self> {
        Ok(CgatsWriter::File(BufWriter::new(File::create(file)?)))
    }

    fn stdout() -> Self {
        CgatsWriter::Stdout(BufWriter::new(stdout()))
    }
}

type WriteResult<T> = std::result::Result<T, std::io::Error>;

impl std::io::Write for CgatsWriter {
    fn write(&mut self, buf: &[u8]) -> WriteResult<usize> {
        match self {
            CgatsWriter::File(bwf) => bwf.write(buf),
            CgatsWriter::Stdout(stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> WriteResult<()> {
        match self {
            CgatsWriter::File(bwf) => bwf.flush(),
            CgatsWriter::Stdout(stdout) => stdout.flush(),
        }
    }
}

#[derive(Debug)]
pub struct Config<'a> {
    command: Command,
    de_method: DEMethod,
    de_report: bool,
    files: Vec<String>,
    matches: ArgMatches<'a>,
    output: CgatsWriter,
}

impl<'a> Config<'a> {
    pub fn new() -> Result<Self> {
        let matches = cli::build_cli().get_matches();
        let cmd_name = matches.subcommand_name();
        let subcommand = cmd_name.unwrap_or_default();
        let submatches = matches.subcommand_matches(subcommand);
        let command = Command::from_string(subcommand);

        let (de_method, de_report, output) = if let Some(subcmd) = submatches {
            let method = DEMethod::from_str(subcmd.value_of("DEMETHOD").unwrap_or("DE2000")).unwrap_or_default();
            let report = subcmd.is_present("DEREPORT");
            let file = subcmd.value_of("OUTPUTFILE");

            let out = if let Some(file) = file {
                CgatsWriter::file(file)?
            } else {
                CgatsWriter::stdout()
            };

            (method, report, out)
        } else {
            (DEMethod::default(), false, CgatsWriter::stdout())
        };


        let files = match cmd_name {
            Some(_cmd) => submatches.expect("SUBCOMMAND")
                .values_of("FILES").unwrap_or_default()
                .map(String::from)
                .collect::<Vec<_>>(),
            None => matches.values_of("FILES").unwrap_or_default()
                .map(String::from)
                .collect::<Vec<_>>(),
        };

        Ok(Self { command, de_method, de_report, files, matches, output})
    }

    pub fn usage(&self) -> String {
        self.matches.usage().to_owned()
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn execute(&mut self) -> Result<()> {
        let cgv = CgatsVec::from_files(&self.files);

        match self.command {
            Command::Display => {
                for cgo in cgv.collection.iter() {
                    writeln!(self.output, "{:?}", cgo)?;
                }
            },

            Command::Print => {
                for cgo in cgv.collection.iter() {
                    writeln!(self.output, "{}", cgo)?;
                }
            }

            Command::Average => {
                write!(self.output, "{}", cgv.average()?)?;
                
            },

            Command::Delta => {
                let cgd = cgv.deltae(self.de_method)?;

                writeln!(self.output, "{}", &cgd)?;
                self.output.flush()?;

                if self.de_report {
                    write!(stderr(), "{}", DeReport::new(&cgd)?)?;
                }
            },

            Command::Cat => {
                write!(self.output, "{}", cgv.concatenate()?)?;
            }
        }

        Ok(())
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Config {
            command: Command::default(),
            de_method: DEMethod::default(),
            de_report: false,
            files: Vec::new(),
            matches: ArgMatches::default(),
            output: CgatsWriter::Stdout(BufWriter::new(stdout())),
        }
    }
}

impl<'a> fmt::Display for Config<'a> {
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