use super::*;
use clap::ArgMatches;

use std::fmt;

#[derive(Debug)]
pub enum Command {
    Average,
    Cat,
    // Convert,
}

impl Command {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "average" | "avg" => Some(Command::Average),
            "concatenate" | "cat" | "append" => Some(Command::Cat),
            // "convert"         => Some(Command::Convert),
            _ => None
        }
    }

    pub fn execute(&self, cgv: CgatsVec) -> CgatsResult<CgatsObject> {
        match &self {
            Command::Average => cgv.average(),
            Command::Cat => cgv.concatenate(),
        }
    }

    pub fn display(&self) -> String {
        let s = format!("{}", &self).to_lowercase();
        format!("{}", s)
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Debug)]
pub struct Config {
    pub command: Option<Command>,
    pub files: Vec<String>,
}

impl Config {
    pub fn build(matches: &ArgMatches) -> Self {
        let cmd_name = matches.subcommand_name();
        let command = match cmd_name {
            Some(cmd) => Command::from_string(cmd),
            None => None
        };

        let files = if let Some(cmd) = cmd_name {
            match matches.subcommand_matches(cmd) {
                Some(scm) => scm.values_of("comparefiles")
                    .expect("Did not find 'comparefiles'")
                    .map(|m| m.to_string())
                    .collect(),
                None => Vec::new()
            }
        } else {
            let clap_files = matches.values_of("files");
            match clap_files {
                Some(f) => f.map(|m| m.to_string()).collect(),
                None => Vec::new()
            }
        };

        Self { command, files }
    }

    pub fn execute(&self) -> CgatsResult<CgatsObject> {
        match &self.command {
            Some(cmd) => cmd.execute(self.cgats_vec()?),
            None => Err(CgatsError::InvalidCommand)
        }
    }

    pub fn cgats_vec(&self) -> CgatsResult<CgatsVec> {
        CgatsVec::from_files(&self.files)
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