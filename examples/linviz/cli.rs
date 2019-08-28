use clap::{App, Arg};

const CRATE_VERSION: &str = env!("CRATE_VERSION");

pub fn app() -> App<'static, 'static> {
    App::new("linviz")
        .author(crate_authors!())
        .version(CRATE_VERSION)
        .about("Visualize ColorBurst linearization CGATS files with GNUPlot")
        .arg(Arg::with_name("normalize")
            .long("normalize")
            .short("n")
            .help("Normalize density values on a scale from 0 to 1 before plotting")
            .takes_value(false))
        .arg(Arg::with_name("clear")
            .help("Clear axes after each plot")
            .long("clear")
            .short("c")
            .takes_value(false))
        .arg(Arg::with_name("files")
            .help("ColorBurst CGATS files to plot")
            .required(true)
            .multiple(true))
}