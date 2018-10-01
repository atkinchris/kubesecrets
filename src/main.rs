#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

mod commands;
mod kubectl;
mod secrets;

use clap::{AppSettings, Arg, SubCommand};

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = app_from_crate!()
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("pull")
                .setting(AppSettings::ArgRequiredElseHelp)
                .about("Pull secrets from kubernetes to a JSON file.")
                .long_about("This command gets all secrets from k8s, and outputs them to the JSON file specified.")
                .arg(Arg::with_name("output").help("output file").required(true)),
        ).get_matches();

    match matches.subcommand() {
        ("pull", Some(pull_matches)) => {
            let output = pull_matches.value_of("output").unwrap();
            return commands::pull(output.to_string());
        }
        _ => unreachable!(),
    }
}
