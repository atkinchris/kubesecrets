#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate base64;
#[macro_use]
extern crate text_io;
extern crate ansi_term;
extern crate subprocess;

mod b64;
mod commands;
mod difference;
mod duplicates;
mod errors;
mod fs;
mod kubectl;
mod secrets;

use ansi_term::Colour::Red;
use clap::{AppSettings, Arg, SubCommand};
use errors::ApplicationError;

fn app() -> Result<(), ApplicationError> {
    let matches = app_from_crate!()
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("pull")
                .about("Pull secrets from kubernetes")
                .arg(Arg::from_usage("-o, --output [FILE] 'output to file'"))
                .arg(Arg::from_usage("-a, --all 'get all secrets'")),
        )
        .subcommand(
            SubCommand::with_name("push")
                .about("Push secrets to kubernetes")
                .arg(
                    Arg::with_name("input")
                        .help("input file containing secrets")
                        .required(true),
                )
                .arg(
                    Arg::from_usage("-p, --prune")
                        .help("prune managed secrets on kubernetes that are not in the input"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("pull", Some(pull_matches)) => {
            let output = pull_matches.value_of("output");
            let get_all = pull_matches.is_present("all");
            return commands::pull(get_all, output);
        }
        ("push", Some(push_matches)) => {
            let input = push_matches.value_of("input").unwrap();
            let prune = push_matches.is_present("prune");
            return commands::push(input, prune);
        }
        _ => unreachable!(),
    }
}

fn main() {
    if let Err(err) = app() {
        eprintln!("\n{}", Red.paint(err.description()));
        std::process::exit(1);
    }
}
