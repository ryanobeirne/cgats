use clap::{
    crate_version,
    crate_authors,
    crate_description,
    App, Arg, SubCommand
};

pub fn build_cli() -> App<'static, 'static> {
    App::new("cgats")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(cgats_files(false))
        .subcommand(SubCommand::with_name("average")
            .alias("avg")
            .about("Average 2 or more CGATS color files")
            .arg(cgats_files(true))
            .arg(output()))
        .subcommand(SubCommand::with_name("cat")
            .aliases(&["concatenate", "concat", "append"])
            .about("Concatenate 2 or more CGATS color files")
            .arg(cgats_files(true))
            .arg(output()))
        .subcommand(SubCommand::with_name("delta")
            .aliases(&["de", "deltae"])
            .about("Calculate the Delta E between each sample in two CGATS files")
            .arg(cgats_files(true))
            .arg(output())
            .arg(Arg::with_name("DEMETHOD")
                .value_name("DE_METHOD")
                .takes_value(true)
                .short("m")
                .long("method")
                .help("Delta E method to use in the calculations")
                .possible_values(&["2000", "1994", "1994t", "cmc1", "cmc2", "1976"]))
            .arg(Arg::with_name("DEREPORT")
                .takes_value(false)
                .short("r")
                .long("report")
                .help("Print Delta E statistical report")))
}

fn output() -> Arg<'static, 'static> {
    Arg::with_name("OUTPUTFILE")
        .value_name("FILE")
        .takes_value(true)
        .short("f")
        .long("output-file")
        .help("Output results to <FILE>")
        .multiple(false)
}

fn cgats_files(req: bool) -> Arg<'static, 'static> {
    Arg::with_name("FILES")
        .help("CGATS files")
        .value_name("FILE")
        .required(req)
        .multiple(true)
}