#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate base64;

mod b64;
mod commands;
mod fs;
mod kubectl;
mod secrets;

use clap::{AppSettings, Arg, SubCommand};

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = app_from_crate!()
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("pull")
                .about("Pull secrets from kubernetes to a JSON file.")
                .long_about("This command gets all secrets from k8s, and outputs them to the JSON file specified.")
                .arg(Arg::from_usage("-o, --output [FILE] 'output to file'"))
                .arg(Arg::from_usage("-a, --all 'get all secrets'")),
        ).get_matches();

    match matches.subcommand() {
        ("pull", Some(pull_matches)) => {
            let output = pull_matches.value_of("output");
            let get_all = pull_matches.is_present("all");
            return commands::pull(get_all, output);
        }
        _ => unreachable!(),
    }
}
