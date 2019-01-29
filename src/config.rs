use super::*;
use clap::ArgMatches;
use std::str::FromStr;

use std::fmt;

#[derive(Debug)]
pub enum Command {
    Average,
    Cat,
    Delta,
    // Convert,
}

impl Command {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "average" | "avg" => Some(Command::Average),
            "concatenate" | "cat" | "append" => Some(Command::Cat),
            "delta" | "deltae" | "de" => Some(Command::Delta),
            // "convert"         => Some(Command::Convert),
            _ => None
        }
    }

    pub fn execute(&self, cmd_opts: &CmdOpts, cgv: CgatsVec) -> CgatsResult<Cgats> {
        match &self {
            Command::Average => cgv.average(),
            Command::Cat => cgv.concatenate(),
            Command::Delta => cgv.deltae(deltae::DEMethod::from_str(&cmd_opts[0])?),
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

pub type CmdOpts = Vec<String>;

#[derive(Debug)]
pub struct Config {
    pub command: Option<Command>,
    pub cmd_opts: CmdOpts,
    pub files: Vec<String>,
}

impl Config {
    pub fn build(matches: &ArgMatches) -> Self {
        let cmd_name = matches.subcommand_name();
        let command = match cmd_name {
            Some(cmd) => Command::from_string(cmd),
            None => None
        };

        let mut cmd_opts = Vec::<String>::new();

        let files = if let Some(cmd) = cmd_name {
            match matches.subcommand_matches(cmd) {
                Some(scm) => {
                    cmd_opts.push(
                        scm.value_of("method").unwrap_or("de2000").to_string()
                    );
                    scm.values_of("comparefiles")
                    .expect("Did not find 'comparefiles'")
                    .map(|s| s.to_string())
                    .collect()
                },
                None => Vec::new()
            }
        } else {
            let clap_files = matches.values_of("files");
            match clap_files {
                Some(f) => f.map(|s| s.to_string()).collect(),
                None => Vec::new()
            }
        };


        Self { command, cmd_opts, files }
    }

    pub fn collect(&self) -> CgatsResult<Cgats> {
        match &self.command {
            Some(cmd) => cmd.execute(&self.cmd_opts, self.cgats_vec()),
            None => Err(CgatsError::InvalidCommand)
        }
    }

    pub fn cgats_vec(&self) -> CgatsVec {
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