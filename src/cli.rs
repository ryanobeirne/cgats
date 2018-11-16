use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("cgats")
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
                .required(true)))
}